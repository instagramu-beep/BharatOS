//! BharatOS libhal block storage — SATA, NVMe, USB mass storage
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const BLOCK_SIZE: u64 = 512;
pub const MAX_DEVICES: usize = 32;

#[derive(Clone, Copy, PartialEq)]
pub enum StorageKind { Sata, Nvme, Usb, Sd, Mmc, Virtual }

#[repr(C)]
pub struct BlockDevice {
    pub id: u8,
    pub kind: StorageKind,
    pub sector_size: u16,
    pub sector_count: u64,
    pub base_addr: u64,
    pub irq: u8,
    pub dma: bool,
    pub ncq: bool,
    pub name: [u8; 32],
}

static mut BLOCK_DEVICES: [Option<BlockDevice>; MAX_DEVICES] = unsafe { core::mem::zeroed() };
static mut BLOCK_COUNT: usize = 0;

pub fn enumerate_storage() {
    unsafe {
        BLOCK_DEVICES[0] = Some(BlockDevice {
            id: 0,
            kind: StorageKind::Virtual,
            sector_size: 512,
            sector_count: 0,
            base_addr: 0,
            irq: 0,
            dma: false,
            ncq: false,
            name: [b'V', b'i', b'r', b't', b'u', b'a', b'l', b'B', b'o', b'o', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        });
        BLOCK_COUNT = 1;
    }
}

pub fn get_block_device(id: u8) -> Option<&'static mut BlockDevice> {
    unsafe { BLOCK_DEVICES.get_mut(id as usize).and_then(|d| d.as_mut()) }
}

pub fn register_device(dev: BlockDevice) {
    unsafe {
        if BLOCK_COUNT < MAX_DEVICES {
            BLOCK_DEVICES[BLOCK_COUNT] = Some(dev);
            BLOCK_COUNT += 1;
        }
    }
}
