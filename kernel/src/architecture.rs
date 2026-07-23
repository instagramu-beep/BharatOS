//! BharatOS kernel architecture module
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct ArchitectureCapabilities {
    pub has_sse: bool,
    pub has_sse2: bool,
    pub has_sse3: bool,
    pub has_ssse3: bool,
    pub has_sse41: bool,
    pub has_sse42: bool,
    pub has_avx: bool,
    pub has_avx2: bool,
    pub has_fma: bool,
    pub has_aes: bool,
    pub has_pclmul: bool,
    pub has_rdrand: bool,
    pub has_nx: bool,
    pub has_lm: bool,
    pub has_smp: bool,
    pub has_apic: bool,
    pub has_x2apic: bool,
    pub has_pae: bool,
    pub has_pse: bool,
    pub has_pse36: bool,
    pub has_pge: bool,
    pub has_mca: bool,
    pub has_cmov: bool,
    pub has_pat: bool,
    pub has_rdtscp: bool,
    pub has_tsc_deadline: bool,
    pub has_tsc: bool,
    pub has_vme: bool,
    pub has_fpu: bool,
}

impl ArchitectureCapabilities {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            let cpuid = unsafe { core::arch::x86_64::__cpuid(1) };
            let mut caps = Self::new();
            caps.has_fpu = (cpuid.edx & (1 << 0)) != 0;
            caps.has_vme = (cpuid.edx & (1 << 1)) != 0;
            caps.has_de = (cpuid.edx & (1 << 2)) != 0;
            caps.has_pse = (cpuid.edx & (1 << 3)) != 0;
            caps.has_tsc = (cpuid.edx & (1 << 4)) != 0;
            caps.has_msr = (cpuid.edx & (1 << 5)) != 0;
            caps.has_pae = (cpuid.edx & (1 << 6)) != 0;
            caps.has_mce = (cpuid.edx & (1 << 7)) != 0;
            caps.has_cmov = (cpuid.edx & (1 << 15)) != 0;
            caps.has_pge = (cpuid.edx & (1 << 13)) != 0;
            caps.has_pse36 = (cpuid.edx & (1 << 17)) != 0;
            caps.has_sse = (cpuid.edx & (1 << 25)) != 0;
            caps.has_sse2 = (cpuid.edx & (1 << 26)) != 0;
            caps.has_apic = (cpuid.edx & (1 << 9)) != 0;
            caps.has_mca = (cpuid.edx & (1 << 14)) != 0;
            caps.has_nx = (cpuid.edx >> 29) & 1 != 0;
            caps.has_lm = (cpuid.edx >> 29) & 1 != 0;
            caps.sse3 = (cpuid.ecx & (1 << 0)) != 0;
            caps.has_ssse3 = (cpuid.ecx & (1 << 9)) != 0;
            caps.has_sse41 = (cpuid.ecx & (1 << 19)) != 0;
            caps.has_sse42 = (cpuid.ecx & (1 << 20)) != 0;
            caps.has_avx = (cpuid.ecx & (1 << 28)) != 0;
            caps.has_aes = (cpuid.ecx & (1 << 25)) != 0;
            caps.has_pclmul = (cpuid.ecx & (1 << 1)) != 0;
            caps.has_rdrand = (cpuid.ecx & (1 << 30)) != 0;
            caps
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self::new()
        }
    }
}

pub fn cores_init() {
    // Initialize per-core data structures
}

pub fn init_arch() {
    let caps = ArchitectureCapabilities::detect();
    // Use capabilities for optimization
}

pub fn boot_id() -> u64 {
    // Return boot-specific unique ID
    0
}

pub fn cpu_ticks() -> u64 {
    libhal::tsc::TscInfo::detect().read()
}

pub fn ipi_send(cpu_mask: u64, vector: u8) {
    unsafe {
        let lapic = libhal::apic::read_apic_base() as *mut u32;
        libhal::apic::write_register(lapic, libhal::apic::LAPIC_ICR_LO,
            (vector as u32) | (1 << 14) | ((cpu_mask as u32) << 24));
    }
}
