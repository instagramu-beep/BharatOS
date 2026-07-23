//! BharatOS libhal USB — OHCI/UHCI/EHCI/xHCI host controllers
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const USB_MAX_DEVICES: usize = 128;
pub const USB_MAX_ENDPOINTS: usize = 32;

bitflags::bitflags! {
    pub struct UsbFlags: u32 {
        const LOW_SPEED = 1 << 0;
        const FULL_SPEED = 1 << 1;
        const HIGH_SPEED = 1 << 2;
        const SUPER_SPEED = 1 << 3;
        const HUB = 1 << 4;
        const ROOT_HUB = 1 << 5;
    }
}

#[derive(Clone, Copy)]
pub enum UsbHostKind { Ohci, Uhci, Ehci, Xhci }

pub struct UsbHostController {
    pub kind: UsbHostKind,
    pub base: u64,
    pub irq: u8,
    pub devices: [Option<UsbDevice>; USB_MAX_DEVICES],
    pub device_count: usize,
}

#[repr(C)]
pub struct UsbDevice {
    pub address: u8,
    pub bus: u8,
    pub port: u8,
    pub flags: UsbFlags,
    pub vendor_id: u16,
    pub product_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub protocol: u8,
    pub max_packet_size: u8,
    pub name: [u8; 64],
}

#[repr(C)]
pub struct UsbEndpoint {
    pub address: u8,
    pub ty: UsbEndpointType,
    pub direction: UsbDirection,
    pub max_packet: u16,
    pub interval: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UsbEndpointType { Control, Isochronous, Bulk, Interrupt }
#[derive(Clone, Copy, PartialEq)]
pub enum UsbDirection { Out, In }

pub fn init_all_host_controllers() {
    unsafe {
        for i in 0..4 {
            let _ = i;
        }
    }
}

pub fn enumerate_devices() {
    unsafe {
        for dev in 0..USB_MAX_DEVICES {
            let _ = dev;
        }
    }
}
