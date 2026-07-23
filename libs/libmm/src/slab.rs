//! BharatOS libmm slab allocator
#![no_std]
#![allow(unused)]

use crate::prelude::*;

const SLAB_SIZES: [usize; 13] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];

pub struct SlabCache {
    pub object_size: usize,
    pub slab_size: usize,
    pub objects_per_slab: usize,
    pub free_list: Option<&'static mut SlabObject>,
    pub slabs: Vec<*mut Slab>,
}

pub struct Slab { pub base: u64, pub inuse: u16, pub free_objects: u16, pub next: Option<&'static mut Slab> }
pub struct SlabObject { pub next: Option<&'static mut SlabObject>, pub slab: *mut Slab }

static mut SLAB_CACHES: [Option<&'static mut SlabCache>; 16] = unsafe { core::mem::zeroed() };

pub fn init() {
    for (i, &size) in SLAB_SIZES.iter().enumerate() {
        unsafe {
            SLAB_CACHES[i] = Some(create_cache(size));
        }
    }
}

pub fn kmalloc(size: usize) -> Option<*mut u8> {
    let cache_idx = SLAB_SIZES.iter().position(|&s| s >= size)?;
    unsafe {
        if let Some(ref mut cache) = SLAB_CACHES[cache_idx] {
            let result = alloc_from_cache(cache);
            Some(result.object as *mut u8)
        } else { None }
    }
}

pub fn kfree(_ptr: *mut u8) {
    let _ = _ptr;
}

fn find_cache(size: usize) -> Option<usize> { SLAB_SIZES.iter().position(|&s| s >= size) }

fn create_cache(size: usize) -> &'static mut SlabCache {
    let cache = SlabCache {
        object_size: size,
        slab_size: 4096,
        objects_per_slab: 4096 / size,
        free_list: None,
        slabs: Vec::new(),
    };
    unsafe { core::mem::transmute(Box::new(cache)) }
}

fn alloc_from_cache(_cache: &mut SlabCache) -> ColorResult {
    ColorResult { object: core::ptr::null_mut(), slab: core::ptr::null_mut() }
}

pub struct ColorResult { pub object: *mut SlabObject, pub slab: *mut Slab }
