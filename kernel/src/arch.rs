#![no_std]
#![allow(unused)]

pub mod x86_64;
pub mod aarch64;
pub mod riscv64;
pub use x86_64::*;

pub mod init;
pub mod timer;
pub mod clk;
pub mod rtc;
pub mod acpi;
pub mod memory;
pub mod mm;
pub mod cpu;
pub mod saver;
pub mod cpu_local;

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub mod gdt;
    pub mod idt;        // (in libhal)
    pub mod pic;
    pub mod apic;
    pub mod interrupts;
    pub mod msr;
    pub mod tsc;
    pub mod paging;
    pub mod syscalls;
    pub mod hpet;
    pub mod serial;
    pub mod fpu;
    pub mod kmm;
    pub mod arch_adapter;
}

// Module alias so `arch::x86_64::gdt::Gdt` always resolves to real chars
pub use self::x86_64::gdt;
pub use self::x86_64::idt;
pub use self::x86_64::pic;
pub use self::x86_64::apic;
pub use self::x86_64::interrupts;
pub use self::x86_64::msr;
pub use self::x86_64::tsc;
pub use self::x86_64::paging;
pub use self::x86_64::serial;

// Architecture info struct used by boot diagnostics
#[derive(Debug, Clone, Copy)]
pub struct ArchInfo {
    pub core_id: u16,
    pub lapic_id: u8,
    pub apic_base: u64,
    pub flags: u64,
}

pub fn cpu_count() -> usize {
    x86_64::apic::lapic_count()
}

pub fn halt_cpu() {
    unsafe { core::arch::asm!("hlt") };
}

pub fn resume_cpu() {
    // wake via mwait or interrupt
    unsafe { core::arch::asm!("mfence") };
}