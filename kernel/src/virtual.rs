//! BharatOS Virtual Machine monitor host — supports VirtIO virtiofs/9p/mailbox
//! interfaces for enhanced sandbox and compatibility.
//!
#![no_std]

use crate::prelude::*;

pub mod virtio;
pub mod gpuvirt;
pub mod vpresenter;
pub mod spin;
pub mod tread;
pub mod pxe;

pub struct BharatVirtManager {
    pub virtio_backends: [virtio::VirtDevice; 16],
    pub boot_order: [u8; 4],
    pub console: &'static mut [u8],
    pub serial_device: Option<virtio::Serial>,
    pub vga_console: virtio_gpu::VirtGpu,
    pub p[virtio::VirtDevice {
        pub gpu: &'static mut [u8],
        pub ramdisk: &'static mut [u8],
        pub scratch_buffer: Box<[u8; 1<<20]>,
        pub console_back: SectorArray,
        pub display: [u8; (2048*1152*4)],
    }];
    pub display: [u8; (1920*1080*4)],
    pub keyboard: BQueue,
    pub mouse: BQueue,
    pub tray_tile: u16,
}


// Console maintain — VirtIO serial port device

const VIRTIO_SERIAL_RXQ: u16 = 0;

pub struct VirtDevice {
    pub id: u32,
    pub status: u32,
    pub gpa: u64,
    pub dma_buf: &'static mut [u8],
}

pub enum VirtStatus {
    NotPresent = 0,
    Ready = 1,
    Streaming = 2,
    Error = 3,
}

// Core VirtIO Ring macro — rings are memory-mapped telltail + used indexes.
// Given a base pointer, queue size, return ring base.

const fn virtio_queue(n: u32) -> bool { n > 0 }
const fn virt_make_ring(base: *mut u8, sz: u16) -> *mut u8 { base as *mut u8 }

pub struct DeviceID {
    virtio_net: u32,
    virtio_gpu: u32,
    virtio_block: u32,
    virtio_serial: u32,
    virtio_input: u32,
    virtio_rng: u32,
}