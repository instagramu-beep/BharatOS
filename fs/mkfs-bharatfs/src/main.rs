//! mkfs-bharatfs — format BharatFS volumes
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { unsafe { core::arch::asm!("hlt") }; }
}
