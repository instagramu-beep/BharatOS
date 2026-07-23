//! BharatOS libcore x86_64 I/O port access
#![no_std]
#![allow(unused)]

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") ret, options(nostack, preserves_flags));
    ret
}

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    core::arch::asm!("in ax, dx", in("dx") port, out("ax") ret, options(nostack, preserves_flags));
    ret
}

#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") val, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    core::arch::asm!("in eax, dx", in("dx") port, out("eax") ret, options(nostack, preserves_flags));
    ret
}

#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn io_wait() { outb(0x80, 0); }

#[inline(always)]
pub fn disable_interrupts() -> u64 {
    let flags: u64;
    unsafe { core::arch::asm!("pushfq; pop {0}; cli", out(reg) flags, options(nostack)); }
    flags
}

#[inline(always)]
pub fn restore_interrupts(flags: u64) {
    unsafe { core::arch::asm!("push {0}; popfq", in(reg) flags, options(nostack)); }
}
