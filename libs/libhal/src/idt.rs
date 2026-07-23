//! BharatOS libhal IDT (Interrupt Descriptor Table) — 256 entries
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::arch::{inb, outb};

const GDT_KERNEL_CODE: u16 = 0x08;
const IDT_ENTRIES: usize = 256;

#[repr(C, packed)]
pub struct IdtEntry {
    pub base_low: u16,
    pub selector: u16,
    pub _zero_and_flags: u16,
    pub base_mid: u16,
    pub base_high: u32,
    pub _reserved: u32,
}

impl IdtEntry {
    pub const NULL: Self = Self {
        base_low: 0, selector: 0, _zero_and_flags: 0,
        base_mid: 0, base_high: 0, _reserved: 0,
    };

    pub fn new(isr: unsafe fn()) -> Self {
        let addr = isr as u64;
        Self {
            base_low: (addr & 0xFFFF) as u16,
            selector: GDT_KERNEL_CODE,
            _zero_and_flags: 0x8E,
            base_mid: ((addr >> 16) & 0xFFFF) as u16,
            base_high: ((addr >> 32) & 0xFFFFFFFF) as u32,
            _reserved: 0,
        }
    }
}

#[repr(C, packed)]
pub struct Idt {
    pub entries: [IdtEntry; IDT_ENTRIES],
    pub size: u16,
}

impl Idt {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::NULL; IDT_ENTRIES],
            size: (IDT_ENTRIES * 16 - 1) as u16,
        }
    }

    pub unsafe fn load(&self) {
        let ptr = (self as *const Self, 2 + self.size as usize);
        core::arch::asm!(
            "lidt [{0}]",
            in(reg) &(self as *const Self as *const u16),
            options(att_syntax, readonly)
        );
    }

    pub fn set(&mut self, idx: usize, isr: unsafe fn()) {
        self.entries[idx] = IdtEntry::new(isr);
    }
}

static mut IDT_TABLE: Idt = Idt::new();

pub fn idt_init() {
    unsafe {
        IDT_TABLE = Idt::new();
        crate::interrupts::isr_setup_early(&mut IDT_TABLE);
        IDT_TABLE.load();
    }
}

pub fn idt_set_handler(idx: usize, isr: unsafe fn()) {
    unsafe { IDT_TABLE.set(idx, isr); }
}
