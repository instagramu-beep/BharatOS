//! BharatOS libhal EOI — End of interrupt
#![no_std]
#![allow(unused)]

pub fn eoi(vector: u8) {
    if vector >= 0x20 {
        unsafe {
            let base = crate::apic::read_apic_base() as *mut u32;
            core::ptr::write_volatile(base.add(0xB0 / 4), 0);
        }
    }
}
