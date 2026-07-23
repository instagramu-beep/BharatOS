//! BharatOS libcore raw memory operations and debug utilities
#![no_std]
#![allow(unused)]

use core::{ptr, slice};

pub fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { ptr::copy_nonoverlapping(src, dst, n); }
    dst
}

pub fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { ptr::copy(src, dst, n); }
    dst
}

pub fn memset(dst: *mut u8, val: u8, n: usize) -> *mut u8 {
    unsafe { ptr::write_bytes(dst, val, n); }
    dst
}

pub fn memzero(dst: *mut u8, n: usize) -> *mut u8 { memset(dst, 0, n) }

pub fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let d = *a.add(i) as i32 - *b.add(i) as i32;
            if d != 0 { return d; }
        }
        0
    }
}

pub fn strlen(s: *const u8) -> usize {
    unsafe { (0..).take_while(|&i| *s.add(i) != 0).count() }
}

pub fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let n = strlen(src);
        ptr::copy_nonoverlapping(src, dst, n + 1);
        dst
    }
}

pub fn strncpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        let len = strlen(src).min(n);
        ptr::copy_nonoverlapping(src, dst, len);
        if len < n { *dst.add(len) = 0; }
        dst
    }
}

pub fn cpu_relax() { unsafe { core::arch::asm!("pause") } }
pub fn io_relax() { unsafe { core::arch::asm!("pause") } }

#[inline(always)]
pub fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

#[inline(always)]
pub fn align_down(val: usize, align: usize) -> usize {
    val & !(align - 1)
}

#[inline(always)]
pub fn is_aligned(val: usize, align: usize) -> bool {
    (val & (align - 1)) == 0
}

#[inline(always)]
pub fn div_round_up(a: usize, b: usize) -> usize { (a + b - 1) / b }

pub fn offset_of<T, F>(f: fn(&T) -> *const F) -> usize {
    unsafe { (f as *const () as usize) - (core::ptr::null::<T>() as usize) }
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self { Self { start, end } }
    pub fn len(&self) -> u64 { self.end.saturating_sub(self.start) }
    pub fn contains(&self, addr: u64) -> bool { addr >= self.start && addr < self.end }
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}
