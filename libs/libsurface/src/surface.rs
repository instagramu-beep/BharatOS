//! BharatOS libsurface — GPU surface for GPU-rendered windows and widgets
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod surface;
pub mod texture;
pub mod shader;
pub mod compositor;

bitflags::bitflags! {
    pub struct SurfaceUsage: u32 {
        const COLOR_ATTACHMENT = 1 << 0;
        const DEPTH_STENCIL    = 1 << 1;
        const SAMPLED          = 1 << 2;
        const STORAGE          = 1 << 3;
        const TRANSFER_SRC     = 1 << 4;
        const TRANSFER_DST     = 1 << 5;
    }
}

#[repr(C)]
pub struct SurfaceDesc {
    pub width: u32,
    pub height: u32,
    pub format: SurfaceFormat,
    pub usage: SurfaceUsage,
    pub samples: u8,
    pub mip_levels: u8,
}

pub enum SurfaceFormat {
    R8G8B8A8Unorm,
    B8G8R8A8Unorm,
    R16G16B16A16Float,
    R32G32B32A32Float,
    D24UnormS8Uint,
    D32Float,
}

impl SurfaceFormat {
    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            Self::R8G8B8A8Unorm => 4,
            Self::B8G8R8A8Unorm => 4,
            Self::R16G16B16A16Float => 8,
            Self::R32G32B32A32Float => 16,
            Self::D24UnormS8Uint => 4,
            Self::D32Float => 4,
        }
    }
}

pub struct Surface {
    pub id: SurfaceId,
    pub descriptor: SurfaceDesc,
    pub pixels: Option<&'static mut [u32]>,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub attached: Option<AttachInfo>,
}

pub type SurfaceId = u64;

pub struct AttachInfo {
    pub window: WindowId,
    pub z_order: u16,
    pub opacity: f32,
    pub transform: Transform,
}

#[derive(Clone, Copy)]
pub enum Transform { None, Rotate90, Rotate180, Rotate270, FlipH, FlipV }
