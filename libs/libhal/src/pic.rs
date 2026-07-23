//! BharatOS libhal PIC — 8259 legacy interrupt controller
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const PIC1_CMD: u16 = 0x20;
pub const PIC1_DATA: u16 = 0x21;
pub const PIC2_CMD: u16 = 0xA0;
pub const PIC2_DATA: u16 = 0xA1;

#[inline(always)]
pub unsafe fn disable_legacy_pic() {
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}

#[inline(always)]
pub unsafe fn remap(off1: u8, off2: u8) {
    outb(PIC1_CMD, 0x11);
    outb(PIC1_DATA, off1);
    outb(PIC1_DATA, 0x04);
    outb(PIC1_DATA, 0x01);
    outb(PIC2_CMD, 0x11);
    outb(PIC2_DATA, off2);
    outb(PIC2_DATA, 0x02);
    outb(PIC2_DATA, 0x01);
    outb(PIC1_DATA, 0);
    outb(PIC2_DATA, 0);
}
