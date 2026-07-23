//! BharatOS kernel boot module
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub version: u32,
    pub memory_map: u64,
    pub memory_map_size: u32,
    pub framebuffer: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,
    pub acpi_root: u64,
    pub cpu_count: u32,
}

#[repr(C)]
pub struct BootMemMap {
    pub entries: [BootMemEntry; 256],
    pub count: usize,
}

#[repr(C)]
pub struct BootMemEntry {
    pub base: u64,
    pub len: u64,
    pub ty: MemoryType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MemoryType { Usable, Reserved, AcpiReclaim, Mmio, Kernel, KernelStack }

impl BootInfo {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn parse(data: &[u8]) -> Result<Self> {
        let info = unsafe { *(data.as_ptr() as *const Self) };
        if info.magic != 0x42484F53545F4F53 { return Err(crate::err::Error::InvalidMagic); }
        Ok(info)
    }
}
