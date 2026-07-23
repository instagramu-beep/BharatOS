//! BharatOS libhal x86_64 architecture module — page tables, GDT, segments
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod gdt;
pub mod idt;
pub mod pic;
pub mod apic;
pub mod interrupts;
pub mod msr;
pub mod tsc;
pub mod paging;
pub mod serial;
pub mod fpu;
pub mod syscalls;
pub mod kmm;

pub mod nice;

/// Boot-time identity page tables
#[repr(C, align(4096))]
pub struct PageTable([u64; 512]);

impl PageTable {
    pub const fn new() -> Self {
        Self([0; 512])
    }

    pub fn set_entry(&mut self, idx: usize, phys: u64, flags: PageFlags) {
        self.0[idx] = (phys & 0x000F_FFFF_FFFF_F000) | flags.bits();
    }

    pub fn get_entry(&self, idx: usize) -> Option<u64> {
        if self.0[idx] & PageFlags::PRESENT.bits() != 0 {
            Some(self.0[idx])
        } else {
            None
        }
    }
}

bitflags::bitflags! {
    pub struct PageFlags: u64 {
        const PRESENT    = 1 << 0;
        const WRITABLE   = 1 << 1;
        const USER       = 1 << 2;
        const HUGE       = 1 << 3;
        const GLOBAL     = 1 << 4;
        const NO_EXEC    = 1 << 5;
        const COW        = 1 << 6;
        const ENCRYPTED  = 1 << 7;
        const DEVICE     = 1 << 8;
        const SWAPPED    = 1 << 9;
        const FILE_BACKED = 1 << 10;
        const LAZY       = 1 << 11;
    }
}

#[repr(C)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

impl GdtEntry {
    pub const NULL: Self = Self {
        limit_low: 0, base_low: 0, base_middle: 0,
        access: 0, granularity: 0, base_high: 0,
    };

    pub fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: gran,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
pub struct Gdt {
    pub entries: [GdtEntry; 8],
    pub limit: u16,
    pub base: usize,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            entries: [GdtEntry::NULL; 8],
            limit: (8 * 8 - 1) as u16,
            base: 0,
        }
    }

    pub unsafe fn load(&self) {
        let ptr = self as *const Self;
        core::arch::asm!("lgdt [{0}]", in(reg) ptr, options(att_syntax, readonly));
    }

    pub fn setup(&mut self) {
        // GDT layout:
        // 0x00: NULL
        // 0x08: Kernel Code  (CS for kernel)
        // 0x10: Kernel Data  (DS for kernel)
        // 0x18: User Code    (CS for userland)
        // 0x20: User Data    (DS for userland)
        // 0x28: TSS
        self.entries[1] = GdtEntry::new(0, 0xFFFFF, 0x9A, 0xAF); // Kernel Code
        self.entries[2] = GdtEntry::new(0, 0xFFFFF, 0x92, 0xCF); // Kernel Data
        self.entries[3] = GdtEntry::new(0, 0xFFFFF, 0xFA, 0xAF); // User Code
        self.entries[4] = GdtEntry::new(0, 0xFFFFF, 0xF2, 0xCF); // User Data
    }
}

#[repr(C)]
pub struct Tss {
    pub reserved1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved3: u64,
    pub iomap_base: u16,
}

static mut BOOT_PML4: PageTable = PageTable([0; 512]);

pub fn setup_paging() {
    unsafe {
        let pml4 = &mut BOOT_PML4;
        // Identity map first 4 MB
        for i in 0..1024 {
            pml4.set_entry(i, (i * 4096) as u64, PageFlags::PRESENT | PageFlags::WRITABLE);
        }
        // Load PML4
        core::arch::asm!(
            "mov cr3, {0}",
            in(reg) pml4,
            options(nostack, preserves_flags)
        );
    }
}

pub fn enable_paging() {
    unsafe {
        let cr0: u64;
        core::arch::asm!("mov {0}, cr0", out(reg) cr0);
        core::arch::asm!("mov cr0, {0}", in(reg) cr0 | (1 << 31) | (1 << 0));
    }
}
