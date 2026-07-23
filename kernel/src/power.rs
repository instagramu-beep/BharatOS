//! BharatOS Power Manager — handles CPU frequency scaling, display power,
//! battery monitoring, wake-source attribution.
#![no_std]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct PowerFlags: u64 {
        const CPU_SCALING        = 1 << 0;
        const GPU_RUNTIME_PM    = 1 << 1;
        const USB_SUSPEND       = 1 << 2;
        const AUDIO_SUSPEND     = 1 << 3;
        const DISPLAY_SUSPEND   = 1 << 4;
        const NET_RUNTIME_PM    = 1 << 5;
        const NVME_APST         = 1 << 6;
        const SSDP_AUTO         = 1 << 7;
        const WOL               = 1 << 8;
        const POLL_30HZ         = 1 << 9;
        const POLL_1HZ          = 1 << 10;
        const ACPI_SLEEP        = 1 << 11;
        const RUNTIME_D3        = 1 << 12;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerProfile {
    Performance, Balanced, Battery, Eco, Presentation, Flight, Custom(u16, u16, u16, u16, u16)
}

#[repr(C)]
pub struct PowerState {
    pub flags: PowerFlags,
    pub power_profile: PowerProfile,
    pub battery_level: u8,
    pub is_charging: bool,
    pub watt_hours: u16,
    pub discharge_rate: i16,
    pub estimated_remaining: u32,
    pub cpu_ghz_base: u16,
    pub cpu_ghz_cur: u16,
    pub gpu_power_limit: u16,
    pub display_brightness: u16,
    pub hue_temp_k: u16,
    pub display_off_timeout_sec: u32,
    pub sleep_timeout_sec: u32,
    pub last_acpi_wake: u128,
}

pub struct PowerManager {
    pub state: PowerState,
    pub governor: fn(),
    pub charging: bool,
    pub suspend_req: SyncMutex<bool>,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            state: PowerState {
                flags: PowerFlags::all(),
                power_profile: PowerProfile::Balanced,
                battery_level: 0,
                is_charging: false,
                watt_hours: 50,
                discharge_rate: 0,
                estimated_remaining: 0,
                cpu_ghz_base: 0,
                cpu_ghz_cur: 0,
                gpu_power_limit: 0,
                display_brightness: 100,
                hue_temp_k: 6500,
                display_off_timeout_sec: 300,
                sleep_timeout_sec: 900,
                last_acpi_wake: 0,
            },
            governor: Self::balance_governor,
            charging: false,
            suspend_req: SyncMutex::new(false),
        }
    }

    pub fn apply_profile(&mut self, p: PowerProfile) {
        self.state.power_profile = p;
        match p {
            PowerProfile::Performance => {
                self.state.cpu_ghz_cur = self.state.cpu_ghz_base;
                self.state.gpu_power_limit = 100;
                self.state.brightness(100);
                self.state.power_flags.remove(PowerFlags::POLL_1HZ);
                self.state.power_flags.insert(PowerFlags::POLL_30HZ);
            }
            PowerProfile::Balanced => {
                self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.8) as u16;
                self.state.gpu_power_limit = 80;
                self.state.brightness((self.state.battery_level + 20).min(100) as u16);
                self.state.power_flags.insert(PowerFlags::POLL_1HZ);
                self.state.power_flags.remove(PowerFlags::POLL_30HZ);
            }
            PowerProfile::Battery => {
                self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.5) as u16;
                self.state.gpu_power_limit = 50;
                self.state.brightness((self.state.battery_level / 3) as u16);
                self.state.power_flags.insert(PowerFlags::POLL_1HZ);
            }
            PowerProfile::Flight => {
                self.state.power_flags.insert(PowerFlags::POLL_30HZ);
                self.state.display_off_timeout_sec = 60;
                self.state.sleep_timeout_sec = 60;
            }
            _ => {}
        }
        self.cpu_freq(self.state.cpu_ghz_cur);
        self.display_power(self.state.display_brightness);
        notify_power_change();
    }

    pub fn balance_governor(&mut self) {
        let bp = self.battery_percent();
        if bp < 5 {
            // Emergency power-BIOS
            self.apply_profile(PowerProfile::Eco);
        } else if bp > 85 && self.charging {
            self.apply_profile(PowerProfile::Performance);
        } else {
            self.apply_profile(PowerProfile::Balanced);
        }
    }

    fn battery_percent(&self) -> u8 {
        if self.state.watt_hours == 0 { return 100; }
        let pct = (self.state.battery_level as u16 * 100 / self.state.watt_hours) as u8;
        pct.max(0).min(100)
    }

    fn cpu_freq(&mut self, ghz: u16) {
        if let Some(msr) = libhal::msr::IA32_PERF_CTL {
            unsafe { msr::write(msr::IA32_PERF_CTL, ghz as u64) };
        }
    }

    fn display_power(&mut self, brightness_pct: u16) {
        let brightness = (brightness_pct * 255 / 100) as u16;
        unsafe {
            // Write to EDID brightness register / ACPI backlight
            acpi::set_backlight(brightness as u8);
        }
        if brightness_pct < 10 {
            self.state.power_flags.insert(PowerFlags::DISPLAY_SUSPEND);
        }
    }

    fn brightness_set(&mut self, val: u16) {
        self.state.display_brightness = val;
        self.display_power(val);
    }
}

// Battery charge levels enum
#[repr(u8)]
pub enum BatteryStatus {
    Full,
    Charging,
    Discharging,
    Empty,
    Unknown,
}

// Wattage thresholds for model selection
pub struct BatteryMetrics {
    pub wh_res: u32,      // mWh remaining
    pub wh_total: u32,    // mWh full
    pub percent: u8,
    pub mins_left: u32,
    pub ma_draw: i16,
}
