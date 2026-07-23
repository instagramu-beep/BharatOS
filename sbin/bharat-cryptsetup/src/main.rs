//! BharatOS cryptsetup — disk encryption setup
#![no_std]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { unsafe { core::arch::asm!("hlt") }; }
}
