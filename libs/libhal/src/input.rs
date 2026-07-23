//! BharatOS libhal input devices
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct InputFlags: u32 {
        const ABSOLUTE = 1 << 0; const RELATIVE = 1 << 1; const MULTITOUCH = 1 << 2;
        const PRESSURE = 1 << 3; const TILT = 1 << 4; const KEYBOARD = 1 << 6; const BUTTON = 1 << 7;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputDeviceType { Keyboard, Mouse, Touch, Pen, Gamepad, Unknown }

#[repr(C)]
pub struct InputDevice {
    pub id: u32,
    pub name: [u8; 64],
    pub vendor_id: u16,
    pub product_id: u16,
    pub ty: InputDeviceType,
    pub flags: InputFlags,
}

pub fn enumerate() {
    unsafe {
        let _ = 0u8;
    }
}
