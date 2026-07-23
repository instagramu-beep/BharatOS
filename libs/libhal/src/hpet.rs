//! BharatOS libhal HPET (High Precision Event Timer)
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const HPET_BASE: u64 = 0xFED00000;
pub const HPET_REG_CAP: usize = 0x00;
pub const HPET_REG_CONF: usize = 0x10;
pub const HPET_REG_MAIN: usize = 0xF0;
pub const HPET_REG_T0_CONF: usize = 0x100;
pub const HPET_REG_T0_CMP: usize = 0x108;

pub struct Hpet {
    pub base: u64,
    pub frequency: u64,
    pub period_fs: u64,
}

impl Hpet {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    pub fn init(base: u64) -> Self {
        let cap = unsafe { (base as *const u64).add(HPET_REG_CAP / 8).read_volatile() };
        let period_fs = cap >> 32;
        let frequency = 10_000_000_000_000 / period_fs;
        Self { base: base, frequency, period_fs }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        unsafe { (self.base as *const u64).add(HPET_REG_MAIN / 8).read_volatile() }
    }

    pub fn set_timer(&self, timer: u8, value: u64) {
        let conf_reg = HPET_REG_T0_CONF + (timer as usize * 0x20);
        let cmp_reg = HPET_REG_T0_CMP + (timer as usize * 0x20);
        unsafe {
            (self.base as *mut u64).add(conf_reg / 8).write_volatile(0x004C);
            (self.base as *mut u64).add(cmp_reg / 8).write_volatile(value);
        }
    }

    pub fn ns_to_ticks(&self, ns: u64) -> u64 { (ns * self.period_fs) / 10_000_000_000 }
    pub fn ticks_to_ns(&self, ticks: u64) -> u64 { (ticks * 10_000_000_000) / self.period_fs }
}

pub fn init_hpet() {
    unsafe {
        let hpet = Hpet::init(HPET_BASE);
        let _ = hpet;
    }
}
