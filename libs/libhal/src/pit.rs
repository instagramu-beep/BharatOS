//! BharatOS libhal PIT (Programmable Interval Timer)
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::arch::{inb, outb};

pub const PIT_BASE: u16 = 0x40;
pub const PIT_CMD: u16 = 0x43;
pub const PIT_CH0: u16 = 0x00;
pub const PIT_LOHI: u16 = 0x30;
pub const PIT_MODE2: u16 = 0x04;

pub fn set_rate(hz: u64) {
    let divisor = (1_193_182u32 / hz as u32).clamp(1, 65535) as u16;
    unsafe {
        outb(PIT_CMD, PIT_LOHI | PIT_MODE2 | PIT_CH0);
        outb(PIT_BASE, (divisor & 0xFF) as u8);
        outb(PIT_BASE, (divisor >> 8) as u8);
    }
}

pub fn stop() {
    unsafe { outb(PIT_CMD, PIT_LOHI | PIT_CH0); }
}
