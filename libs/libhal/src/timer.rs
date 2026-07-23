//! BharatOS libhal timers — PIT, HPET, TSC, RTC
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::arch::{inb, outb};

pub mod pit;
pub mod hpet;
pub mod tsc;
pub mod rtc;

static mut TICKS: u64 = 0;
static mut FREQUENCY_HZ: u64 = 100;
static mut BOOT_NS: u64 = 0;

#[derive(Clone, Copy)]
pub struct TimeSpec { pub tv_sec: u64, pub tv_nsec: u64 }

pub fn init(frequency_hz: u64) {
    unsafe {
        FREQUENCY_HZ = frequency_hz;
        TICKS = 0;
        BOOT_NS = 0;
    }
    pit::set_rate(frequency_hz);
}

pub fn tick() {
    unsafe {
        TICKS += 1;
        BOOT_NS += 1_000_000_000 / FREQUENCY_HZ;
    }
}

pub fn uptime_ns() -> u64 { unsafe { BOOT_NS } }
pub fn uptime_secs() -> u64 { unsafe { BOOT_NS / 1_000_000_000 } }
pub fn uptime() -> TimeSpec { unsafe { TimeSpec { tv_sec: BOOT_NS / 1_000_000_000, tv_nsec: BOOT_NS % 1_000_000_000 } } }

pub fn sleep_ns(ns: u64) {
    let start = uptime_ns();
    while uptime_ns() - start < ns { unsafe { core::arch::asm!("hlt") }; }
}

pub fn sleep_ms(ms: u64) { sleep_ns(ms * 1_000_000); }
pub fn sleep_secs(secs: u64) { sleep_ns(secs * 1_000_000_000); }

pub fn rdtsc() -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi, options(nostack, preserves_flags));
        ((hi as u64) << 32) | (lo as u64)
    }
}

pub fn monotonic_ns() -> u64 { uptime_ns() }
