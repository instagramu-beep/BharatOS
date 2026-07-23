//! BharatOS kernel utility functions
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::time::timestamp;

pub fn strlen(s: *const u8) -> usize {
    unsafe { (0..).take_while(|&i| *s.add(i) != 0).count() }
}

pub fn strcmp(a: *const u8, b: *const u8) -> i32 {
    unsafe {
        for i in 0.. {
            let ca = *a.add(i);
            let cb = *b.add(i);
            if ca != cb { return (ca as i32) - (cb as i32); }
            if ca == 0 { break; }
        }
        0
    }
}

pub fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { ptr::copy_nonoverlapping(src, dst, n); dst }
}

pub fn memset(dst: *mut u8, val: u8, n: usize) -> *mut u8 {
    unsafe { ptr::write_bytes(dst, val, n); dst }
}

pub fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let d = *a.add(i) as i32 - *b.add(i) as i32;
            if d != 0 { return d; }
        }
        0
    }
}

pub fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let ca = *a.add(i);
            let cb = *b.add(i);
            if ca != cb { return (ca as i32) - (cb as i32); }
            if ca == 0 { break; }
        }
        0
    }
}

pub fn strlen_max(s: *const u8, max: usize) -> usize {
    unsafe { (0..max).take_while(|&i| *s.add(i) != 0).count() }
}

pub fn hex(value: u64, width: usize) -> [u8; 16] {
    const HEX: &[u8] = b"0123456789ABCDEF";
    let mut buf = [b'0'; 16];
    for i in (0..width.min(16)).rev() {
        buf[15 - i] = HEX[((value >> (i * 4)) & 0xF) as usize];
    }
    buf
}

pub fn ptr_align<T>(ptr: *mut T, align: usize) -> *mut T {
    let addr = ptr as usize;
    ((addr + align - 1) & !(align - 1)) as *mut T
}

pub fn is_kernel_addr(addr: u64) -> bool {
    addr >= 0xFFFFFFFF80000000
}

pub fn is_user_addr(addr: u64) -> bool {
    addr < 0x00007FFFFFFFFFFF
}

pub fn copy_from_user(dst: &mut [u8], src: *const u8, n: usize) -> Result<()> {
    if !is_user_addr(src as u64) { return Err(crate::err::Error::PermissionDenied); }
    unsafe { ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), n.min(dst.len())); }
    Ok(())
}

pub fn copy_to_user(dst: *mut u8, src: &[u8], n: usize) -> Result<()> {
    if !is_user_addr(dst as u64) { return Err(crate::err::Error::PermissionDenied); }
    unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst, n.min(src.len())); }
    Ok(())
}

pub fn delay_loop(count: u64) {
    for _ in 0..count { unsafe { core::arch::asm!("nop") }; }
}

pub fn get_timestamp() -> u128 {
    timestamp()
}

pub fn div_round_up(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub fn align_up(val: u64, align: u64) -> u64 {
    (val + align - 1) & !(align - 1)
}
