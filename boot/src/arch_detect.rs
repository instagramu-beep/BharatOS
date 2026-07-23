//! ArchDetect — detects CPU architecture, topology, ACPI tables and features
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ArchitectureInfo {
    pub arch_id: ArchId,
    pub cpu_count: usize,
    pub cpu_features: u64,
    pub apic_base: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchId {
    X86_64,
    AARCH64,
    RISCV64,
    Unknown,
}

impl ArchitectureInfo {
    pub fn new() -> Self {
        // CPUID for x86_64 detection (safe fallback pattern)
        match detect_x86_64() {
            Ok(info) => info,
            Err(_) => match detect_aarch64() {
                Ok(info) => info,
                Err(_) => ArchitectureInfo {
                    arch_id: ArchId::RISCV64,
                    cpu_count: 1,
                    cpu_features: 0,
                    apic_base: 0,
                    page_size: 0x1000,
                }
            }
            ,
        }
    }

    #[inline(always)]
    pub fn cpu_count(&self) -> u32 { self.cpu_count as u32 }
    #[inline(always)]
    pub fn cpu_features(&self) -> u64 { self.cpu_features }
    #[inline(always)]
    pub fn page_size(&self) -> u64 { self.page_size }

    pub fn feature_string(&self) -> [&'static str; 16] {
        let mut flags = [""; 16];
        let f = self.cpu_features;
        let names = ["sse", "sse2", "sse3", "ssse3", "sse4.1", "sse4.2",
                     "avx", "avx2", "fma3", "aes", "pclmul", "rdrand",
                     "vme", "nx", "lm", "fpu"];
        for (i, &name) in names.iter().enumerate() {
            if (f >> i) & 1 == 1 { flags[i] = name; }
        }
        flags
    }
}

fn detect_x86_64() -> Result<ArchitectureInfo, ()> {
    use core::arch::x86_64::__cpuid;
    unsafe {
        let cpuid = __cpuid(1);
        Ok(ArchitectureInfo {
            arch_id: ArchId::X86_64,
            cpu_count: 1,
            cpu_features: (cpuid.ecx as u64)
                | ((cpuid.edx as u64) << 32),
            apic_base: {
                let cpuid7 = __cpuid(7);
                // APIC MSR handled in kernel init
                0
            },
            page_size: 0x1000,
        })
    }
}

fn detect_aarch64() -> Result<ArchitectureInfo, ()> {
    Err(())
}