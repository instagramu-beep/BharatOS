//! BharatOS libhal serial — 16550 UART
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::arch::{inb, outb};

pub const COM1: u16 = 0x3F8;
pub const COM2: u16 = 0x2F8;
pub const COM3: u16 = 0x3E8;
pub const COM4: u16 = 0x2E8;

pub struct SerialPortDriver {
    port: u16,
    initialized: bool,
}

impl SerialPortDriver {
    pub const fn new(port: u16) -> Self {
        Self { port, initialized: false }
    }

    pub fn init(&mut self) {
        if self.initialized { return; }
        unsafe {
            outb(self.port + 1, 0x00);
            outb(self.port + 3, 0x80);
            outb(self.port + 0, 0x03);
            outb(self.port + 1, 0x00);
            outb(self.port + 3, 0x03);
            outb(self.port + 2, 0xC7);
            outb(self.port + 4, 0x0B);
        }
        self.initialized = true;
    }

    pub fn send(&mut self, byte: u8) {
        unsafe { outb(self.port, byte); }
    }

    pub fn receive(&mut self) -> Option<u8> {
        unsafe {
            if (inb(self.port + 5) & 1) != 0 { Some(inb(self.port)) } else { None }
        }
    }

    pub fn is_received(&self) -> bool {
        unsafe { (inb(self.port + 5) & 1) != 0 }
    }

    pub fn is_transmit_empty(&self) -> bool {
        unsafe { (inb(self.port + 5) & 0x20) != 0 }
    }

    pub fn write(&mut self, data: &[u8]) {
        for &byte in data {
            while !self.is_transmit_empty() {}
            self.send(byte);
        }
    }

    pub fn write_str(&mut self, s: &str) { self.write(s.as_bytes()); }
    pub fn write_line(&mut self, line: &str) { self.write_str(line); self.send(b'\n'); }
}

static mut COM1_DRIVER: Option<SerialPortDriver> = None;

pub fn init() {
    unsafe { COM1_DRIVER = Some(SerialPortDriver::new(COM1)); }
    if let Some(ref mut d) = COM1_DRIVER { d.init(); }
}

pub fn com1() -> &'static mut SerialPortDriver {
    unsafe {
        COM1_DRIVER.get_or_insert(SerialPortDriver::new(COM1));
        COM1_DRIVER.as_mut().unwrap()
    }
}

pub fn write(data: &[u8]) { com1().write(data); }
pub fn write_str(s: &str) { com1().write_str(s); }
pub fn write_line(s: &str) { com1().write_line(s); }
