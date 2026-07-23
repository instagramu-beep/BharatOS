//! BharatOS libhal ring0 — lowest-level kernel operations
#![no_std]
#![allow(unused)]

pub fn ring0_init() {
    unsafe {
        disable_interrupts();
        disable_sqe();
    }
}
pub fn enter_ring0() { unsafe { core::arch::asm!("cli") }; }
pub fn set_kernel_stack(stack: u64) {
    unsafe {
        core::arch::asm!("mov {}, rsp", in(reg) stack, options(nostack));
    }
}
pub fn disable_sqe() { unsafe { core::arch::asm!("wrmsr", in("ecx") 0xC001_0115, in("edx") 0u32, in("eax") 0u32); } }
pub fn enable_interrupts() { unsafe { core::arch::asm!("sti") }; }
pub fn disable_interrupts() { unsafe { core::arch::asm!("cli") }; }
pub fn save_flags() -> u64 {
    let flags: u64;
    unsafe { core::arch::asm!("pushfq; pop {0}", out(reg) flags, options(nostack)); }
    flags
}
pub fn restore_flags(flags: u64) { unsafe { core::arch::asm!("push {0}; popfq", in(reg) flags, options(nostack)); } }
pub fn hlt() { unsafe { core::arch::asm!("hlt", options(nostack, preserves_flags)); } }
