//! BharatOS libcore force-idle — CPU idle control
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub fn force_idle(enable: bool) {
    if enable {
        unsafe { core::arch::asm!("hlt"); }
    }
}

pub fn idle_yield() {
    unsafe { core::arch::asm!("hlt"); }
}

pub fn park_thread() {
    loop { unsafe { core::arch::asm!("hlt"); } }
}

pub fn unpark_thread(_tid: u64) {
    // Send interrupt to wake thread
}

pub fn wait_for_event() {
    unsafe { core::arch::asm!("wfe"); }
}
