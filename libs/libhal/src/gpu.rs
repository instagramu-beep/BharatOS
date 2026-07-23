//! BharatOS libhal GPU detection and early framebuffer
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct GpuFlags: u32 {
        const BOOT = 1 << 0;
        const PCI = 1 << 2;
        const PCIE = 1 << 3;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GpuVendor { Intel, AMD, NVIDIA, VMware, QEMU, Bochs, Unknown }

#[repr(C)]
pub struct GpuDevice {
    pub id: u64,
    pub vendor: GpuVendor,
    pub device_id: u16,
    pub flags: GpuFlags,
    pub fb_base: u64,
    pub fb_size: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub pitch: u32,
    pub name: [u8; 64],
    pub vram_size: u64,
}

static mut GPU_DEVICES: [Option<GpuDevice>; 8] = unsafe { core::mem::zeroed() };
static mut GPU_COUNT: usize = 0;

impl GpuDevice {
    pub fn detect_gpus() -> usize {
        unsafe {
            GPU_DEVICES[0] = Some(GpuDevice {
                id: 0,
                vendor: GpuVendor::Unknown,
                device_id: 0,
                flags: GpuFlags::BOOT,
                fb_base: 0,
                fb_size: 0,
                width: 1024,
                height: 768,
                bpp: 32,
                pitch: 4096,
                name: [b'B', b'o', b'o', b't', b' ', b'V', b'G', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vram_size: 0,
            });
            GPU_COUNT = 1;
            1
        }
    }

    pub fn set_mode(&mut self, width: u32, height: u32, bpp: u8) {
        self.width = width;
        self.height = height;
        self.bpp = bpp;
        self.pitch = width * bpp as u32 / 8;
    }
}

pub fn detect_gpus() { let _ = GpuDevice::detect_gpus(); }
pub fn get_gpu_count() -> usize { unsafe { GPU_COUNT } }
pub fn get_gpu(_idx: usize) -> Option<&'static mut GpuDevice> {
    unsafe { GPU_DEVICES.get_mut(0).and_then(|d| d.as_mut()) }
}
pub fn boot_fb() -> Option<&'static mut GpuDevice> { get_gpu(0) }
