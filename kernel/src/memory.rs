//! BharatOS kernel memory management framework
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct PageFlags: u64 {
        const PRESENT     = 1 << 0;
        const WRITABLE    = 1 << 1;
        const USER        = 1 << 2;
        const HUGE        = 1 << 3;
        const GLOBAL      = 1 << 4;
        const NO_EXEC     = 1 << 5;
        const COW         = 1 << 6;
        const ENCRYPTED   = 1 << 7;
        const SWAPPED     = 1 << 9;
        const FILE_BACKED = 1 << 10;
        const LAZY        = 1 << 11;
    }
}

pub struct MemoryManager {
    pub total_pages: u64,
    pub free_pages: u64,
    pub slab_pages: u64,
    pub page_size: u32,
    pub zones: [Zone; 8],
    pub zone_count: usize,
}

#[derive(Clone, Copy)]
pub struct Zone {
    pub start: u64,
    pub len: u64,
    pub free_head: u64,
    pub order: u8,
    pub free_count: u64,
}

static mut MM_INSTANCE: Option<MemoryManager> = None;

impl MemoryManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self, mem_map: &BootMemMap) {
        self.total_pages = 0;
        self.free_pages = 0;
        self.page_size = 4096;

        for entry in mem_map.entries.iter().take(mem_map.count) {
            if entry.ty == MemoryType::Usable || entry.ty == MemoryType::Kernel {
                let pages = entry.len / 4096;
                self.total_pages += pages;
                self.free_pages += pages;
            }
        }
    }

    pub fn alloc_page(&mut self) -> Result<u64> {
        if self.free_pages == 0 { return Err(crate::err::Error::NoMemory); }
        self.free_pages -= 1;
        Ok(0x100000 + self.free_pages * 4096)
    }

    pub fn free_page(&mut self, addr: u64) {
        self.free_pages += 1;
        let _ = addr;
    }
}

pub fn init_mm(mem_map: &BootMemMap) -> Result<()> {
    unsafe {
        MM_INSTANCE = Some(MemoryManager::new());
        if let Some(mm) = MM_INSTANCE.as_mut() {
            mm.init(mem_map);
        }
    }
    Ok(())
}

pub fn get_mm() -> Option<&'static mut MemoryManager> {
    unsafe { MM_INSTANCE.as_mut() }
}
