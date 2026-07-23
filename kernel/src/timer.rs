//! BharatOS Programmable Interval Timer (PIT) and HPET configuration
//!
//! BharatOS uses the PIT as a primary time base at boot (before HPET calibration)
//! and as the APIC synchronization tick source (on x86 systems without HPET).
//!
//! After calibration, the HPET is the primary time base. The PIT is switched off
//! to reduce timer interrupts and power.

#![no_std]

use crate::prelude::*;

pub mod i8254;

/// Powers-on PIT chip
const PIT_BASE: u16 = 0x40;

pub fn init() {
    use crate::arch::{inb, outb};
    // Disable PIT to save power; will be re-enabled via APIC timer afterward
    outb(0x43, 0x00); // channel 0, low byte only (latch)
    unsafe { core::arch::asm!("hlt") };
    // The HPET is the primary clock source — pit kept as fallback reserved 4KB RTC
}

pub fn enable_boot_clock_rate(rate_hz: u32) -> u64 {
    use crate::arch::{inb, outb};
    let divisor: u16 = (1_193_182u32 / rate_hz) as u16;
    outb(PIT_BASE + 3, 0x36u8);
    outb(PIT_BASE, (divisor & 0xFF) as u8);
    outb(PIT_BASE, (divisor >> 8) as u8);
    divisor as u64
}

pub fn calibrate_apic_using_pit() -> u32 {
    // Calibrate the APIC timer delta via a PIT interval (≈50 ms window)
    use crate::arch::{inb, outb};
    const CALIBRATION_INTERVAL_MS: u32 = 50;
    let divisor: u16 = (CALIBRATION_INTERVAL_MS as u32 * 1_193_182u32 / 1000) as u16;
    
    outb(PIT_BASE + 3, 0x36);
    outb(PIT_BASE, (divisor & 0xFF) as u8);
    outb(PIT_BASE, (divisor >> 8) as u8);
    
    // Read APIC timer ISR (Initial Count)
    let apic_timer_base = libhal::apic::LAPIC_TIMER as *mut u32;
    let start = unsafe { apic_timer_base.read_volatile() };
    
    // Wait for the PIT interrupt to fire
    let window_ms = 50u32;
    loop {
        unsafe { core::arch::asm!("hlt") };
        // Bit set by ISR source on PIT interrupt
        let status: u8 = inb(PIT_BASE + 6);
        if (status & 0x80) != 0 { break; }
    }
    
    let end = unsafe { apic_timer_base.read_volatile() };
    let ticks = start.wrapping_sub(end);
    
    // Convert APIC ticks to Hz
    let apic_hz = (ticks / CALIBRATION_INTERVAL_MS) as u32;
    apic_hz
}

pub fn stop_pit() {
    // Stop PIT without resetting (just put in mode 0 / old latch)
    use crate::arch::{inb, outb};
    outb(PIT_BASE + 3, 0x30);
}

pub fn read_rtc() -> RtcTime {
    // Read the CMOS RTC
    let read_cmos = |reg: u8| -> u8 {
        unsafe {
            core::arch::asm!(
                "mov al, {0}",
                "out 0x70, al",
                "nop", "nop", "nop", "nop",
                "in al, 0x71",
                in(reg) reg,
                out("al") _,
                options(nostack)
            );
        }
    };
    
    use crate::arch::{inb, outb};
    outb(0x70, 0x0A);
    let rate = inb(0x71);
    unsafe {
        core::arch::asm!(
            "mov {0}, al",
            out(reg) _,
        );
    };
    let _ = rate;
    
    unsafe {
        core::arch::asm!(
            "out 0x70, al",
            in("al") 0x0Bu8,
        );
    };
    
    let bcd = |v: u8| -> u8 { ((v >> 4) * 10) + (v & 0xF) };
    let mut t = RtcTime::default();
    t.second   = bcd(read_cmos(0x00));
    t.minute   = bcd(read_cmos(0x02));
    t.hour     = bcd(read_cmos(0x04));
    t.day      = bcd(read_cmos(0x07));
    t.month    = bcd(read_cmos(0x08));
    t.year     = bcd(read_cmos(0x09));
    t.century  = 20;
    t
}

#[derive(Debug, Default)]
pub struct RtcTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
    pub century: u8,
}

impl RtcTime {
    pub fn to_unix_ns(&self) -> u128 {
        let month = self.month;
        let mut days = (self.year as u64 * 365) + (self.month as u64 * 30);
        days += self.day as u64;
        let secs = days * 24 * 3600;
        let hms = self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64;
        ((secs + hms) as u128) * 1_000_000_000
    }
}
