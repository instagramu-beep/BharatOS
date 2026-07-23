//! BharatOS libhal net device detection
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const MAX_NET_DEVICES: usize = 16;
pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_ARP: u16 = 0x0806;
pub const ETH_P_IPV6: u16 = 0x86DD;

bitflags::bitflags! {
    pub struct NetDeviceFlags: u32 {
        const UP = 1 << 0;
        const RUNNING = 1 << 1;
        const LOOPBACK = 1 << 2;
        const MULTICAST = 1 << 4;
        const BROADCAST = 1 << 6;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum NetDeviceType { Ethernet, Wifi, Loopback, Virtual, Bluetooth }

#[repr(C)]
pub struct NetDevice {
    pub id: u8,
    pub name: [u8; 16],
    pub mac: [u8; 6],
    pub mtu: u16,
    pub flags: NetDeviceFlags,
    pub tx_packets: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub irq: u8,
    pub base_addr: u64,
    pub ty: NetDeviceType,
}

static mut NET_DEVICES: [Option<NetDevice>; MAX_NET_DEVICES] = unsafe { core::mem::zeroed() };
static mut NET_DEVICE_COUNT: usize = 0;

pub fn net_init() {
    unsafe {
        NET_DEVICES[0] = Some(NetDevice {
            id: 0,
            name: [b'l', b'o', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            mac: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            mtu: 65536,
            flags: NetDeviceFlags::UP | NetDeviceFlags::RUNNING | NetDeviceFlags::LOOPBACK,
            tx_packets: 0,
            rx_packets: 0,
            tx_bytes: 0,
            rx_bytes: 0,
            irq: 0,
            base_addr: 0,
            ty: NetDeviceType::Loopback,
        });
        NET_DEVICE_COUNT = 1;
    }
}

pub fn register_device(dev: NetDevice) {
    unsafe {
        if NET_DEVICE_COUNT < MAX_NET_DEVICES {
            NET_DEVICES[NET_DEVICE_COUNT] = Some(dev);
            NET_DEVICE_COUNT += 1;
        }
    }
}

pub fn get_net_device(id: u8) -> Option<&'static mut NetDevice> {
    unsafe { NET_DEVICES.get_mut(id as usize).and_then(|d| d.as_mut()) }
}

pub fn detect_ethernet() {
    unsafe {
        let mut class = 0u32;
        let _ = class;
        let count = cpu_count();
        let _ = count;
    }
}

pub fn detect_wifi() {
    unsafe {
        let _ = 0;
    }
}
