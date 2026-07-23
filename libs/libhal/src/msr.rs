//! BharatOS libhal MSR (Model Specific Registers) for x86_64
#![no_std]
#![allow(unused)]

pub const IA32_EFER: u32 = 0xC0000080;
pub const IA32_STAR: u32 = 0xC0000081;
pub const IA32_LSTAR: u32 = 0xC0000082;
pub const IA32_FMASK: u32 = 0xC0000084;
pub const IA32_FS_BASE: u32 = 0xC0000100;
pub const IA32_GS_BASE: u32 = 0xC0000101;
pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;
pub const IA32_PAT: u32 = 0x277;
pub const IA32_PERF_CTL: u32 = 0x199;
pub const IA32_APIC_BASE: u32 = 0x1B;
pub const IA32_PLATFORM_INFO: u32 = 0xCE;
pub const IA32_PERF_GLOBAL_CTRL: u32 = 0x38F;

pub const EFER_LME: u64 = 1 << 8;
pub const EFER_LMA: u64 = 1 << 10;
pub const EFER_NXE: u64 = 1 << 11;
pub const EFER_SVME: u64 = 1 << 12;
pub const PAT_DEFAULT: u64 = 0x0007040600070406;

#[inline(always)]
pub fn read(msr: u32) -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") lo, out("edx") hi,
            options(nostack, preserves_flags)
        );
    }
    ((hi as u64) << 32) | (lo as u64)
}

#[inline(always)]
pub fn write(msr: u32, val: u64) {
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") (val & 0xFFFFFFFF) as u32,
            in("edx") (val >> 32) as u32,
            options(nostack, preserves_flags)
        );
    }
}
