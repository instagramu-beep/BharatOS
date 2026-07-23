//! BharatOS libinput — input device subsystem
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod keyboard;
pub mod mouse;
pub mod touch;
pub mod pen;
pub mod gamepad;
pub mod gestur;

bitflags::bitflags! {
    pub struct InputFlags: u32 {
        const ABSOLUTE = 1 << 0;
        const RELATIVE = 1 << 1;
        const MULTITOUCH = 1 << 2;
        const PRESSURE = 1 << 3;
        const TILT = 1 << 4;
        const WHEEL = 1 << 5;
        const KEYBOARD = 1 << 6;
        const BUTTON = 1 << 7;
        const HAPTIC = 1 << 8;
    }
}
