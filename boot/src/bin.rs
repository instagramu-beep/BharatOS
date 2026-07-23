//! BharatOS UEFI Bootloader — main binary
#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main(_image: Handle, mut st: SystemTable<Boot>) -> Status {
    let _ = st.boot_services().set_watchdog_timer(0, 0, None);
    st.stdout().write_str("BharatOS Bootloader v1.0\n").unwrap();
    Status::SUCCESS
}
