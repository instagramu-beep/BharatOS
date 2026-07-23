//! BharatOS libhal APIC — Advanced Programmable Interrupt Controller
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const LAPIC_ID: u32 = 0x20;
pub const LAPIC_VERSION: u32 = 0x30;
pub const LAPIC_TPR: u32 = 0x80;
pub const LAPIC_APR: u32 = 0x90;
pub const LAPIC_PPR: u32 = 0xA0;
pub const LAPIC_EOI: u32 = 0xB0;
pub const LAPIC_TIMER: u32 = 0x320;
pub const LAPIC_TIMER_INIT: u32 = 0x380;
pub const LAPIC_TIMER_DIV: u32 = 0x3E0;
pub const LAPIC_SVR: u32 = 0xF0;
pub const LAPIC_ESR: u32 = 0x280;
pub const LAPIC_CMCI: u32 = 0x2F0;
pub const LAPIC_PCINT: u32 = 0x340;
pub const LAPIC_LINT0: u32 = 0x350;
pub const LAPIC_LINT1: u32 = 0x360;
pub const LAPIC_ERROR: u32 = 0x370;
pub const LAPIC_ICR_LO: u32 = 0x300;
pub const LAPIC_ICR_HI: u32 = 0x310;
pub const LAPIC_SELF_IPI: u32 = 0x3F0;

#[inline(always)]
pub fn read_apic_base() -> u64 {
    crate::msr::read(crate::msr::IA32_APIC_BASE)
}

#[inline(always)]
pub fn lapic_init() {
    unsafe {
        let apic = (read_apic_base() & !0xFFF) as *mut u32;
        write_register(apic, LAPIC_SVR, 0x100 | 0x20);
        write_register(apic, LAPIC_TPR, 0);
        write_register(apic, LAPIC_TIMER_DIV, 0xB);
        write_register(apic, LAPIC_TIMER, 0xFFFFFFFF);
    }
}

#[inline(always)]
pub fn ioapic_init() {
    unsafe {
        if let Some(ioapic) = crate::acpi::find_ioapic(0) {
            let base = ioapic.address as *mut u32;
            let version = read_ioapic(base, 0x01);
            let entries = ((version >> 16) & 0xFF) + 1;
            for i in 0..entries {
                write_ioapic(base, (i << 1) | 0x10, (i + 0x20) << 8);
            }
        }
    }
}

#[inline(always)]
pub fn eoi() {
    unsafe {
        let apic = (read_apic_base() & !0xFFF) as *mut u32;
        core::ptr::write_volatile(apic.add((LAPIC_EOI / 4) as usize), 0);
    }
}

#[inline(always)]
pub unsafe fn read_register(base: *mut u32, reg: u32) -> u32 {
    core::ptr::read_volatile(base.add((reg / 4) as usize))
}

#[inline(always)]
pub unsafe fn write_register(base: *mut u32, reg: u32, val: u32) {
    core::ptr::write_volatile(base.add((reg / 4) as usize), val);
}

#[inline(always)]
pub unsafe fn read_ioapic(base: *mut u32, reg: u32) -> u32 {
    core::ptr::write_volatile(base, reg as u32);
    read_register(base, 0x10)
}

#[inline(always)]
pub unsafe fn write_ioapic(base: *mut u32, reg: u32, val: u32) {
    core::ptr::write_volatile(base, reg as u32);
    core::ptr::write_volatile(base.add(0x10 / 4), val);
}
