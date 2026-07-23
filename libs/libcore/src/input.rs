//! BharatOS libcore input device handling
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct InputDeviceFlags: u32 {
        const ABSOLUTE   = 1 << 0;
        const RELATIVE   = 1 << 1;
        const MULTITOUCH = 1 << 2;
        const PRESSURE   = 1 << 3;
        const TILT       = 1 << 4;
        const WHEEL      = 1 << 5;
        const KEYBOARD   = 1 << 6;
        const BUTTON     = 1 << 7;
        const HAPTIC     = 1 << 8;
        const LED        = 1 << 9;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputDevice {
    pub id: InputDeviceId,
    pub name: [u8; 64],
    pub vendor_id: u16,
    pub product_id: u16,
    pub ty: InputDeviceType,
    pub flags: InputDeviceFlags,
    pub axes: [AxisInfo; 8],
    pub axis_count: u8,
    pub buttons: [ButtonInfo; 16],
    pub button_count: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct AxisInfo {
    pub code: u16,
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonInfo {
    pub code: u16,
    pub pressed: bool,
    pub value: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputDeviceType { Keyboard, Mouse, Touch, Pen, Gamepad, Joystick, Unknown }

pub type InputDeviceId = u32;

#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    pub device: InputDeviceId,
    pub kind: InputEventKind,
    pub code: u16,
    pub value: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum InputEventKind {
    KeyDown, KeyUp,
    PointerMove, PointerDown, PointerUp,
    TouchBegin, TouchMove, TouchEnd,
}

pub struct InputManager {
    pub devices: Vec<InputDevice>,
    pub event_queue: Vec<InputEvent>,
}

impl InputManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        let _ = self;
    }
    pub fn poll(&mut self) -> Option<InputEvent> { self.event_queue.pop() }
    pub fn register(&mut self, dev: InputDevice) { self.devices.push(dev); }
    pub fn get_device(&self, id: InputDeviceId) -> Option<&InputDevice> {
        self.devices.iter().find(|d| d.id == id)
    }
}
