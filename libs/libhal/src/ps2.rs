//! BharatOS libhal PS/2 keyboard and mouse
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::arch::{inb, outb};

pub const PS2_CMD: u16 = 0x64;
pub const PS2_DATA: u16 = 0x60;
pub const PS2_STATUS_OUT: u8 = 0x01;
pub const PS2_STATUS_IN: u8 = 0x02;
pub const PS2_STATUS_AUX: u8 = 0x20;

bitflags::bitflags! {
    pub struct KeyboardFlags: u32 {
        const EXTENDED = 1 << 0;
        const RELEASED = 1 << 1;
        const LEFT_SHIFT = 1 << 2;
        const RIGHT_SHIFT = 1 << 3;
        const LEFT_CTRL = 1 << 4;
        const RIGHT_CTRL = 1 << 5;
        const LEFT_ALT = 1 << 6;
        const RIGHT_ALT = 1 << 7;
        const CAPS_LOCK = 1 << 8;
        const NUM_LOCK = 1 << 9;
        const SCROLL_LOCK = 1 << 10;
    }
}

pub struct Ps2Keyboard {
    pub flags: KeyboardFlags,
    pub extended: bool,
    pub released: bool,
}

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Self { flags: KeyboardFlags::empty(), extended: false, released: false }
    }

    pub fn init(&mut self) {
        unsafe {
            outb(PS2_CMD, 0xAD);
            outb(PS2_CMD, 0xAE);
            outb(PS2_CMD, 0x20);
            let config = inb(PS2_DATA);
            outb(PS2_CMD, 0x60);
            outb(PS2_DATA, config | 0x01);
        }
    }

    pub fn is_key_available(&self) -> bool {
        unsafe { (inb(PS2_CMD) & PS2_STATUS_OUT) != 0 }
    }

    pub fn read_scancode(&mut self) -> Option<u8> {
        if self.is_key_available() {
            let code = unsafe { inb(PS2_DATA) };
            self.handle(code);
            Some(code)
        } else { None }
    }

    fn handle(&mut self, code: u8) {
        if code == 0xE0 { self.extended = true; return; }
        if code == 0xE1 { return; }
        self.released = (code & 0x80) != 0;
        self.extended = false;
    }
}

static mut KEYBOARD: Option<Ps2Keyboard> = None;

pub fn keyboard_init() -> &'static mut Ps2Keyboard {
    unsafe {
        KEYBOARD.get_or_insert(Ps2Keyboard::new());
        KEYBOARD.as_mut().unwrap()
    }
}

pub fn init() { let _ = keyboard_init(); }
