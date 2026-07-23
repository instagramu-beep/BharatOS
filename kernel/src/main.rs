//! BharatOS Kernel — main binary
#![no_std]
#![no_main]
#![allow(unused)]

use core::panic::PanicInfo;
use libcore::prelude::*;
use libcore::kernel_log;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_log!(crate::kernel::logger::LogLevel::Fatal, "PANIC", "{}", info);
    loop { unsafe { core::arch::asm!("hlt") }; }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::entry::_bharat_kernel_entry()
}
