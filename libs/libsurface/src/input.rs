//! BharatOS libsurface input devices
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
        const VOICE      = 1 << 10;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputDeviceDesc {
    pub variant_id: u32,
    pub ty: InputDeviceType,
    pub flags: InputDeviceFlags,
    pub name: [u8; 64],
    pub vendor_id: u16,
    pub product_id: u16,
    pub version: u16,
    pub bus: u8,
    pub axes: [AxisInfo; 8],
    pub axis_count: u8,
    pub buttons: [ButtonInfo; 16],
    pub button_count: u8,
    pub led_count: u8,
    pub force_feedback: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct AxisInfo {
    pub code: u16,
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub flat: i32,
    pub fuzz: i32,
    pub resolution: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonInfo {
    pub code: u16,
    pub pressed: bool,
    pub value: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputDeviceType {
    Unknown = 0,
    Keyboard = 1,
    Mouse = 2,
    Touch = 3,
    Pen = 4,
    Gamepad = 5,
    Joystick = 6,
    Remote = 7,
    Wheel = 8,
    Gesture = 9,
    EyeTracker = 10,
    Switch = 11,
}

#[repr(C)]
pub struct InputEvent {
    pub timestamp: u128,
    pub device: InputDeviceId,
    pub kind: InputEventKind,
    pub code: u16,
    pub value: i32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
}

pub type InputDeviceId = u32;

#[derive(Clone, Copy, Debug)]
pub enum InputEventKind {
    KeyDown = 0,
    KeyUp = 1,
    KeyRepeat = 2,
    PointerMove = 3,
    PointerDown = 4,
    PointerUp = 5,
    PointerCancel = 6,
    AxisChanged = 7,
    ButtonChanged = 8,
    TouchBegin = 9,
    TouchMove = 10,
    TouchEnd = 11,
    TouchCancel = 12,
    PenDown = 13,
    PenMove = 14,
    PenUp = 15,
    GestureStart = 16,
    GestureUpdate = 17,
    GestureEnd = 18,
    Connected = 19,
    Disconnected = 20,
}

pub struct InputManager {
    pub devices: [Option<InputDeviceDesc>; 32],
    pub device_count: usize,
    pub active_device: Option<InputDeviceId>,
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
    pub touch: TouchState,
}

#[repr(C)]
pub struct KeyboardState {
    pub keys: [bool; 256],
    pub modifiers: ModifierState,
}

#[repr(C)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
    pub altgr: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
}

#[repr(C)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub x_delta: i32,
    pub y_delta: i32,
    pub wheel_delta: i32,
    pub buttons: u32,
    pub pressed: u32,
}

#[derive(Clone, Copy)]
pub struct TouchState {
    pub contacts: [TouchContact; 10],
    pub contact_count: u8,
    pub gesture: Option<Gesture>,
}

#[derive(Clone, Copy)]
pub struct TouchContact {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub major: f32,
    pub minor: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
    pub tool_type: ToolType,
}

#[derive(Clone, Copy)]
pub enum ToolType { Finger, Stylus, Palm, Unknown }

#[derive(Clone, Copy)]
pub struct Gesture {
    pub kind: GestureKind,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32,
}

#[derive(Clone, Copy)]
pub enum GestureKind {
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
    Pinch,
    Rotate,
    Tap,
    DoubleTap,
    LongPress,
}
