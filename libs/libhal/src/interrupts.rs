//! BharatOS libhal interrupts — ISR registration, IRQ routing, EOI
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub static mut IRQ_HANDLERS: [Option<fn()>; 256] = [None; 256];
pub static mut IRQ_ENABLE_MASK: [u64; 4] = [0; 4];

pub fn isr_setup_early(_idt: &mut crate::idt::Idt) {
    unsafe {
        IRQ_HANDLERS.fill(None);
        IRQ_ENABLE_MASK.fill(0);
    }
}

pub fn register_handler(vector: usize, handler: fn()) {
    unsafe { IRQ_HANDLERS[vector] = Some(handler); }
}

pub fn enable_vector(vector: u8) {
    unsafe {
        IRQ_ENABLE_MASK[(vector / 64) as usize] |= 1 << (vector % 64);
        IRQ_HANDLERS[vector as usize] = Some(default_handler);
    }
}

pub fn disable_vector(vector: u8) {
    unsafe { IRQ_ENABLE_MASK[(vector / 64) as usize] &= !(1 << (vector % 64)); }
}

pub fn enable_all_vectors() {
    unsafe {
        for i in 0..256 {
            if IRQ_HANDLERS[i].is_some() {
                enable_vector(i as u8);
            }
        }
    }
}

pub fn eoi(vector: u8) {
    if vector >= 0x20 {
        unsafe {
            let base = crate::apic::read_apic_base() as *mut u32;
            core::ptr::write_volatile(base.add((crate::apic::LAPIC_EOI / 4) as usize), 0);
        }
    }
}

pub fn handle_interrupt(vector: u8) {
    unsafe {
        if let Some(handler) = IRQ_HANDLERS[vector as usize] {
            handler();
            eoi(vector);
        }
    }
}

fn default_handler() {}
