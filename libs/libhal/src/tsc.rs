//! BharatOS libhal TSC (Time Stamp Counter)
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct TscInfo {
    pub frequency_hz: u64,
    pub invariant: bool,
    pub constant_tsc: bool,
}

impl TscInfo {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    pub fn detect() -> Self {
        let mut info = Self::new();
        info.invariant = true;
        info.constant_tsc = true;
        info.frequency_hz = detect_tsc_freq();
        info
    }

    #[inline(always)]
    pub fn read() -> u64 {
        unsafe {
            let lo: u32;
            let hi: u32;
            core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi, options(nostack, preserves_flags));
            ((hi as u64) << 32) | (lo as u64)
        }
    }

    pub fn ns_to_ticks(&self, ns: u64) -> u64 { (ns * self.frequency_hz) / 1_000_000_000 }
    pub fn ticks_to_ns(&self, ticks: u64) -> u64 { (ticks * 1_000_000_000) / self.frequency_hz }
    pub fn calibrate() -> Self { let mut info = Self::detect(); info }
}

fn detect_tsc_freq() -> u64 { 3_000_000_000 }

static mut TSC_INFO: Option<TscInfo> = None;

pub fn init() { unsafe { TSC_INFO = Some(TscInfo::calibrate()); } }
pub fn get_tsc() -> Option<&'static TscInfo> { unsafe { TSC_INFO.as_ref() } }
