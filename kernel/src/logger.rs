//! BharatOS unified logger — 16-stage boot trace + runtime logs + crash dumps
//!
//! 1. Logger can (optionally) swap endian and buffer by netlink bridge.
//! 2. Boot diagnostics overlay is visible if GPU present and window manager
//!    accepts a BOOTDIAG window_type = BOOTPHASE.
//! 3. The log indices are stored in volatile memory to prevent GC by MM layer.
//!
#![no_std]
#![allow(unused)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::time;
use crate::err::Error;

type RawEntry = [u8; 64];

pub struct LogRing {
    entries: [RawEntry; 1024],
    offset: AtomicUsize,
    epoch: AtomicU64,
    overflow: AtomicU64,
}

pub enum LogLevel { Debug = 0, Info = 1, Warn = 2, Error = 3, Fatal = 4, Init = 5, Ok = 6 }

#[derive(Clone, Copy)]
pub struct LogEntry {
    pub level: LogLevel,
    pub subsystem: [u8; 8],
    pub message: [u8; 44],
    pub timestamp: u128,
    pub cpu: u16,
    pub epoch: u64,
}

impl LogRing {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }
    
    pub fn write(&self, level: LogLevel, subsystem: &[u8], msg: &[u8]) {
        let idx = self.offset.load(Ordering::Relaxed) % 1024;
        let mut entry: RawEntry = [0; 64];
        entry[0] = level as u8;
        entry[1..9].copy_from_slice(&subsystem[0..8]);
        entry[9..53].copy_from_slice(msg.get(0..44).unwrap_or(&[]));
        
        unsafe { core::ptr::write_volatile(self.entries.as_ptr().add(idx).cast(), entry) };
        self.offset.fetch_add(1, Ordering::Relaxed);
    }
}

static LOGGER: LogRing = LogRing::new();

#[track_caller]
pub fn log(level: LogLevel, subsystem: &str, msg: &str) {
    LOGGER.write(level, subsystem.as_bytes(), msg.as_bytes());
    if level as u8 >= LogLevel::Error as u8 {
        serial_output("[SERIAL] ");
        serial_output(subsystem);
        serial_output(": ");
        serial_output(msg);
        serial_output("\n");
    }
}

pub fn init() {
    // Initialize GPA map and flush any stale entries at boot
    // (boot-phase 0 logs may have been written before MM is ready)
}

fn serial_output(s: &str) {
    use crate::arch::x86_64::serial::SerialPort::COM1;
    let mut port = COM1.lock();
    for b in s.bytes() { port.send(b); }
}

#[inline]
pub fn kernel_log!(lvl: LogLevel, $subsys:expr, $($arg:tt)*) {
    let msg = alloc::format!($($arg)*);
    log(lvl, $subsys, &msg);
}

pub trait DisplayLayer {
    fn paint_boot_phase(&self, entries: &[LogEntry], config: DisplayTheme);
}

#[derive(Clone, Copy)]
pub struct DisplayTheme {
    pub background: u32,
    pub text: u32,
    pub highlight: u32,
}