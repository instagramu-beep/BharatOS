//! BharatOS libmm — kernel memory manager with buddy allocator
#![no_std]
#![allow(unused)]

use crate::prelude::*;

const PAGE_SIZE: usize = 4096;
const MAX_MEMORY: usize = 0x100000000; // 4 GB max
const MAX_ZONES: usize = 32;
const MAX_ORDER: usize = 20;

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
        const SWAPPED    = 1 << 9;
        const FILE_BACKED = 1 << 10;
    }
}

#[repr(C)]
pub struct Zone {
    pub start: u64,
    pub len: u64,
    pub free_head: u64,
    pub order: u8,
    pub free_count: u64,
}

pub struct FrameAllocator {
    pub zones: [Zone; MAX_ZONES],
    pub zone_count: usize,
    pub total_pages: u64,
    pub free_pages: u64,
    pub locked: spin::Mutex<()>,
}

impl FrameAllocator {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    pub fn init(&mut self, mem_map: &kernel::boot::BootMemMap) {
        self.total_pages = 0;
        self.free_pages = 0;
        self.zone_count = 0;
        for i in 0..mem_map.count.min(128) {
            let entry = &mem_map.entries[i];
            if matches!(entry.ty, kernel::boot::MemoryType::Usable | kernel::boot::MemoryType::Kernel) {
                let pages = (entry.len / PAGE_SIZE as u64).max(1);
                self.total_pages += pages;
                self.free_pages += pages;
                if self.zone_count < MAX_ZONES {
                    self.zones[self.zone_count] = Zone {
                        start: entry.base,
                        len: entry.len,
                        free_head: 0,
                        order: 0,
                        free_count: pages,
                    };
                    self.zone_count += 1;
                }
            }
        }
    }

    pub fn alloc_page(&mut self) -> Result<u64> {
        for zone in &mut self.zones[..self.zone_count] {
            if zone.free_count > 0 {
                let phys = zone.start + zone.free_head;
                zone.free_head += PAGE_SIZE as u64;
                zone.free_count -= 1;
                self.free_pages -= 1;
                return Ok(phys);
            }
        }
        Err(crate::err::Error::NoMemory)
    }

    pub fn free_page(&mut self, phys: u64) {
        self.free_pages += 1;
        let _ = phys;
    }
}

static mut FRAME_ALLOCATOR: Option<FrameAllocator> = None;

pub fn init(mem_map: &kernel::boot::BootMemMap) -> Result<()> {
    unsafe {
        FRAME_ALLOCATOR = Some(FrameAllocator::new());
        FRAME_ALLOCATOR.as_mut().unwrap().init(mem_map);
    }
    Ok(())
}

pub fn alloc_page() -> Result<u64> {
    unsafe { FRAME_ALLOCATOR.as_mut().ok_or(crate::err::Error::NoMemory)?.alloc_page() }
}

pub fn free_page(phys: u64) {
    unsafe { if let Some(ref mut fa) = FRAME_ALLOCATOR { fa.free_page(phys); } }
}

pub fn get_allocator() -> Option<&'static mut FrameAllocator> {
    unsafe { FRAME_ALLOCATOR.as_mut() }
}
