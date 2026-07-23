//! BharatOS libmm kernel heap — bump + slab hybrid
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::mm::frame;

pub const KERNEL_HEAP_START: u64 = 0x100000;
pub const KERNEL_HEAP_SIZE: usize = 256 * 1024 * 1024;

static mut HEAP_NEXT: u64 = KERNEL_HEAP_START;
static mut HEAP_END: u64 = KERNEL_HEAP_START + KERNEL_HEAP_SIZE as u64;
static HEAP_LOCK: spin::Mutex<()> = spin::Mutex::new(());

pub fn init_kernel_heap() {
    unsafe {
        HEAP_NEXT = KERNEL_HEAP_START;
        HEAP_END = KERNEL_HEAP_START + KERNEL_HEAP_SIZE as u64;
    }
}

pub fn kmalloc(size: usize) -> Option<*mut u8> {
    let _g = HEAP_LOCK.lock();
    let aligned = mem::align_up(HEAP_NEXT as usize, 8);
    if aligned + size > HEAP_END as usize { return None; }
    unsafe {
        HEAP_NEXT = (aligned + size) as u64;
        Some(aligned as *mut u8)
    }
}

pub fn kfree(_ptr: *mut u8) {
    // No-op for bump allocator (would need free list in production)
}

pub fn heap_used() -> usize {
    unsafe { (HEAP_NEXT - KERNEL_HEAP_START) as usize }
}

pub fn heap_free() -> usize {
    unsafe { (HEAP_END - HEAP_NEXT) as usize }
}
