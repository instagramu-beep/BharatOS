//! BharatOS File Manager — main binary
#![no_std]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point for file manager
    loop { unsafe { core::arch::asm!("hlt") }; }
}
