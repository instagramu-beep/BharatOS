//! BharatOS libhal CPU detection and features
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct CpuInfo {
    pub vendor: CpuVendor,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
    pub feature_flags: u64,
    pub extended_flags: u64,
    pub core_count: u8,
    pub thread_count: u8,
    pub base_freq_mhz: u32,
    pub max_freq_mhz: u32,
    pub cache_l1d: u32,
    pub cache_l1i: u32,
    pub cache_l2: u32,
    pub cache_l3: u32,
    pub brand_string: [u8; 48],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CpuVendor { Intel, AMD, ARM, RISCv, Unknown }

impl CpuInfo {
    pub const fn new() -> Self {
        Self {
            vendor: CpuVendor::Unknown,
            family: 0,
            model: 0,
            stepping: 0,
            feature_flags: 0,
            extended_flags: 0,
            core_count: 1,
            thread_count: 1,
            base_freq_mhz: 0,
            max_freq_mhz: 0,
            cache_l1d: 0,
            cache_l1i: 0,
            cache_l2: 0,
            cache_l3: 0,
            brand_string: [0; 48],
        }
    }

    pub fn has_feature(&self, feature: u32) -> bool { (self.feature_flags & (1 << feature)) != 0 }
    pub fn has_extended(&self, feature: u32) -> bool { (self.extended_flags & (1 << feature)) != 0 }
    pub fn frequency(&self) -> u32 { self.base_freq_mhz }
}

static mut CPU_INFO: Option<CpuInfo> = None;

pub fn detect() -> CpuInfo {
    unsafe {
        if let Some(info) = CPU_INFO { return info; }
        let info = detect_cpu();
        CPU_INFO = Some(info);
        info
    }
}

fn detect_cpu() -> CpuInfo {
    let mut info = CpuInfo::new();
    #[cfg(target_arch = "x86_64")]
    {
        let cpuid = unsafe { core::arch::x86_64::__cpuid(1) };
        info.feature_flags = (cpuid.ecx as u64) | ((cpuid.edx as u64) << 32);
        info.family = ((cpuid.eax >> 8) & 0xF) as u8;
        info.model = ((cpuid.eax >> 4) & 0xF) as u8;
        info.stepping = (cpuid.eax & 0xF) as u8;
        info.vendor = if (info.feature_flags & (1 << 13)) != 0 { CpuVendor::Intel } else if (info.feature_flags & (1 << 11)) != 0 { CpuVendor::AMD } else { CpuVendor::Unknown };
        info.core_count = 1;
        info.thread_count = 1;
        info.base_freq_mhz = 2400;
        info.max_freq_mhz = 4800;
        info.cache_l1d = 32 * 1024;
        info.cache_l1i = 32 * 1024;
        info.cache_l2 = 256 * 1024;
        info.cache_l3 = 6 * 1024 * 1024;
        info.brand_string = [b'B', b'h', b'a', b'r', b'a', b't', b'O', b'S', b' ', b'x', b'8', b'6', b'_', b'6', b'4', b' ', b'C', b'P', b'U', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }
    info
}

pub fn cpu_count() -> usize { detect().core_count as usize }
pub fn is_smp_supported() -> bool { detect().has_feature(19) }
pub fn current_cpu() -> usize { 0 }
