//! BharatOS libcore timestamps, monotonic clock, boot-time epoch
#![no_std]
#![allow(unused)]

use core::sync::atomic::{AtomicU64, Ordering};

static BOOT_NS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    pub nanos: u64,
}

impl Duration {
    pub const ZERO: Self = Self { nanos: 0 };
    pub fn from_nanos(n: u64) -> Self { Self { nanos: n } }
    pub fn from_micros(m: u64) -> Self { Self { nanos: m * 1000 } }
    pub fn from_millis(m: u64) -> Self { Self { nanos: m * 1_000_000 } }
    pub fn from_secs(s: u64) -> Self { Self { nanos: s * 1_000_000_000 } }
    pub fn as_nanos(&self) -> u64 { self.nanos }
    pub fn as_secs(&self) -> u64 { self.nanos / 1_000_000_000 }
    pub fn as_millis(&self) -> u64 { self.nanos / 1_000_000 }
}

impl core::ops::Add for Duration {
    type Output = Duration;
    fn add(self, rhs: Duration) -> Duration {
        Duration { nanos: self.nanos + rhs.nanos }
    }
}

impl core::ops::Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Duration {
        Duration { nanos: self.nanos.saturating_sub(rhs.nanos) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Instant {
    pub inner: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self { inner: BOOT_NS.load(Ordering::Relaxed) }
    }
}

impl core::ops::Sub for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Duration {
        Duration { nanos: self.inner.saturating_sub(rhs.inner) }
    }
}

pub fn timestamp() -> u128 {
    BOOT_NS.load(Ordering::Relaxed) as u128
}

pub fn tick(inc_ns: u64) {
    BOOT_NS.fetch_add(inc_ns, Ordering::Relaxed);
}

pub fn sleep(dur: Duration) {
    if dur.nanos == 0 { return; }
    let start = BOOT_NS.load(Ordering::Relaxed);
    let deadline = start + dur.nanos;
    while BOOT_NS.load(Ordering::Relaxed) < deadline {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[inline(always)]
pub fn rdtsc() -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi, options(nostack, preserves_flags));
        ((hi as u64) << 32) | (lo as u64)
    }
}

pub fn monotonic_ns() -> u64 {
    BOOT_NS.load(Ordering::Relaxed)
}
