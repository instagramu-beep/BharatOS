//! BharatOS Settings — main binary
#![no_std]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point for settings
    loop { unsafe { core::arch::asm!("hlt") }; }
}
