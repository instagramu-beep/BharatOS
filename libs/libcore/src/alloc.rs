//! BharatOS libcore alloc — bump allocator for kernel/embedded use
#![no_std]
#![allow(unused)]

extern crate alloc;

pub use alloc::alloc::{GlobalAlloc, Layout};
pub use alloc::boxed::Box;
pub use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
pub use alloc::format;
pub use alloc::rc::Rc;
pub use alloc::string::{String, ToString};
pub use alloc::sync::Arc;
pub use alloc::vec::Vec;
pub use alloc::vec;

use super::mem;
use super::mem::align_up;

struct BharatAllocator {
    next: usize,
    end: usize,
}

impl BharatAllocator {
    const fn new() -> Self { Self { next: 0x100000, end: 0x100000 + 256 * 1024 * 1024 } }
}

unsafe impl GlobalAlloc for BharatAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = align_up(self.next, layout.align());
        if ptr + layout.size() > self.end { return core::ptr::null_mut(); }
        let _ = self.next; // read-only during alloc
        ptr as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

pub struct GLOBAL_ALLOCATOR;
unsafe impl GlobalAlloc for GLOBAL_ALLOCATOR {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        BharatAllocator::new().alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        BharatAllocator::new().dealloc(ptr, layout)
    }
}

#[global_allocator]
static A: GLOBAL_ALLOCATOR = GLOBAL_ALLOCATOR;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init() {
    // Initialize allocator with kernel heap region
}

pub fn vec_from_slice(s: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(s.len());
    v.extend_from_slice(s);
    v
}
