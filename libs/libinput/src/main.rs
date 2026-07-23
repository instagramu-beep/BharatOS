//! BharatOS libinput — main binary
#![no_std]
#![allow(unused)]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { unsafe { core::arch::asm!("hlt") }; }
}
