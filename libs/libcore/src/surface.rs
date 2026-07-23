//! BharatOS libcore surface abstractions
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct SurfaceFlags: u32 {
        const HIDDEN       = 1 << 0;
        const OPAQUE       = 1 << 1;
        const DAMAGED      = 1 << 2;
        const ALPHA        = 1 << 3;
        const FULLSCREEN   = 1 << 4;
        const DOUBLE_BUFFER = 1 << 5;
    }
}

#[derive(Clone, Copy)]
pub enum SurfaceKind {
    Window,
    Offscreen,
    Swapchain,
    Framebuffer,
    Overlay,
}

#[repr(C)]
pub struct SurfaceDesc {
    pub width: u32,
    pub height: u32,
    pub format: SurfaceFormat,
    pub flags: SurfaceFlags,
    pub kind: SurfaceKind,
}

#[repr(C)]
pub struct SurfaceInfo {
    pub id: u64,
    pub desc: SurfaceDesc,
    pub pixels: &'static mut [u8],
    pub pitch: u32,
    pub age: u32,
}

pub enum SurfaceFormat {
    ARGB8888,
    XRGB8888,
    RGB888,
    RGB565,
    RGBA8888,
    BGRA8888,
}

impl SurfaceFormat {
    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            Self::ARGB8888 | Self::XRGB8888 | Self::RGBA8888 | Self::BGRA8888 => 4,
            Self::RGB888 => 3,
            Self::RGB565 => 2,
        }
    }
}

pub struct SurfaceManager;

impl SurfaceManager {
    pub fn create_surface(&mut self, desc: SurfaceDesc) -> Result<SurfaceInfo> {
        let bpp = desc.format.bytes_per_pixel() as usize;
        let size = (desc.width * desc.height * bpp as u32) as usize;
        let pixels = &mut [0u8; 0]; // placeholder
        Ok(SurfaceInfo {
            id: 0,
            desc,
            pixels,
            pitch: desc.width * bpp as u32,
            age: 0,
        })
    }

    pub fn destroy_surface(&mut self, _id: u64) {
        let _ = self;
    }
    pub fn present(&mut self, _id: u64) -> Result<()> { Ok(()) }
    pub fn flip(&mut self, _id: u64) -> Result<u64> { Ok(0) }
}
