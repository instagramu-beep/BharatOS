//! BharatOS libhal power management
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct PowerFlags: u64 {
        const CPU_SCALING = 1 << 0;
        const GPU_PM = 1 << 1;
        const USB_SUSPEND = 1 << 2;
        const AUDIO_SUSPEND = 1 << 3;
        const DISPLAY_PM = 1 << 4;
        const NET_PM = 1 << 5;
        const NVME_APST = 1 << 6;
        const WOL = 1 << 8;
        const ACPI_SLEEP = 1 << 11;
        const RUNTIME_D3 = 1 << 12;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerProfile { Performance, Balanced, Battery, Eco, Flight, Custom }

#[repr(C)]
pub struct PowerState {
    pub flags: PowerFlags,
    pub power_profile: PowerProfile,
    pub battery_level: u8,
    pub is_charging: bool,
    pub watt_hours: u16,
    pub discharge_rate: i16,
    pub cpu_ghz_cur: u16,
    pub cpu_ghz_base: u16,
    pub gpu_power_limit: u16,
    pub display_brightness: u16,
    pub display_off_timeout_sec: u32,
    pub sleep_timeout_sec: u32,
}

pub struct PowerManager {
    pub state: PowerState,
}

impl PowerManager {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    pub fn init(&mut self) {
        self.state = PowerState {
            flags: PowerFlags::all(),
            power_profile: PowerProfile::Balanced,
            battery_level: 0,
            is_charging: false,
            watt_hours: 50,
            discharge_rate: 0,
            cpu_ghz_cur: 2400,
            cpu_ghz_base: 2400,
            gpu_power_limit: 80,
            display_brightness: 100,
            display_off_timeout_sec: 300,
            sleep_timeout_sec: 900,
        };
    }

    pub fn apply_profile(&mut self, p: PowerProfile) {
        self.state.power_profile = p;
        match p {
            PowerProfile::Performance => { self.state.cpu_ghz_cur = self.state.cpu_ghz_base; self.state.gpu_power_limit = 100; self.state.display_brightness = 100; }
            PowerProfile::Balanced => { self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.8) as u16; self.state.gpu_power_limit = 80; self.state.display_brightness = 80; }
            PowerProfile::Battery => { self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.5) as u16; self.state.gpu_power_limit = 50; self.state.display_brightness = 50; }
            PowerProfile::Eco => { self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.3) as u16; self.state.gpu_power_limit = 30; self.state.display_brightness = 30; }
            PowerProfile::Flight => { self.state.cpu_ghz_cur = (self.state.cpu_ghz_base as f32 * 0.25) as u16; self.state.gpu_power_limit = 0; self.state.display_brightness = 20; }
            PowerProfile::Custom => {}
        }
    }

    pub fn set_brightness(&mut self, level: u16) { self.state.display_brightness = level.min(100); }
    pub fn cpu_freq(&self) -> u32 { self.state.cpu_ghz_cur as u32 }
}
