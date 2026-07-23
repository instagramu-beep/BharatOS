//! BharatOS libhal CPU timing and clock sources
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::time::{Duration, timestamp};

pub struct TimingInfo {
    pub boot_time: u128,
    pub uptime: u128,
    pub tsc_freq_hz: u64,
    pub bus_clock_hz: u64,
    pub timer_freq_hz: u64,
}

impl TimingInfo {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn now(&self) -> u128 {
        timestamp()
    }

    pub fn elapsed(&self, since: u128) -> Duration {
        Duration::from_nanos((self.now() - since) as u64)
    }

    pub fn uptime(&self) -> Duration {
        Duration::from_nanos(self.uptime as u64)
    }

    pub fn calib_delay_loop(&self, ms: u32) -> u32 {
        let start = libhal::tsc::TscInfo::detect().read();
        let target = (ms as u64 * 3_000_000) / 1000;
        let mut elapsed;
        loop {
            let now = libhal::tsc::TscInfo::detect().read();
            elapsed = now.wrapping_sub(start);
            if elapsed > target { break; }
        }
        elapsed as u32
    }
}

pub fn boot_timer_init() {
    // Initialize boot-phase timer for diagnostics
}

pub fn delay_us(us: u32) {
    let start = libhal::tsc::TscInfo::detect().read();
    let target = (us as u64 * 3_000_000) / 1_000_000;
    loop {
        let now = libhal::tsc::TscInfo::detect().read();
        if now.wrapping_sub(start) > target { break; }
    }
}

pub fn delay_ms(ms: u32) {
    for _ in 0..ms { delay_us(1000); }
}
