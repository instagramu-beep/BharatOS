//! BharatOS texture management
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub enum TextureFormat {
    R8,
    RG8,
    RGB8,
    RGBA8,
    R16,
    RG16,
    RGB16,
    RGBA16,
    R16F,
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R11G11B10F,
    DXT1,
    DXT5,
    BC7,
    ASTC4x4,
    ASTC8x8,
    R8G8B8A8Unorm,
    B8G8R8A8Unorm,
    SRGB8A8,
    Depth24Stencil8,
    Depth32Float,
}

#[derive(Clone, Copy, Debug)]
pub enum TextureUsage {
    Sampling,
    ColorAttachment,
    DepthStencil,
    Storage,
    TransferSrc,
    TransferDst,
    Sampled,
    StorageBinding,
}

bitflags::bitflags! {
    pub struct TextureFlags: u32 {
        const MIPMAP       = 1 << 0;
        const CUBE_MAP     = 1 << 1;
        const ARRAY        = 1 << 2;
        const USAGE_SAMPLE = 1 << 3;
        const USAGE_COLOR  = 1 << 4;
        const USAGE_DEPTH  = 1 << 5;
        const USAGE_STORE  = 1 << 6;
        const USAGE_TRANSFER = 1 << 7;
    }
}

impl TextureFormat {
    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            Self::R8 | Self::R16 => 1,
            Self::RG8 | Self::RG16 | Self::R16F | Self::Depth24Stencil8 => 2,
            Self::RGB8 | Self::RGB16 | Self::R32F | Self::R11G11B10F => 4,
            Self::RGBA8 | Self::RGBA16 | Self::RGBA32F | Self::DXT1 | Self::R8G8B8A8Unorm | Self::B8G8R8A8Unorm | Self::SRGB8A8 => 4,
            Self::DXT5 | Self::BC7 | Self::ASTC4x4 | Self::Depth32Float => 8,
            Self::ASTC8x8 => 16,
        }
    }
}

#[repr(C)]
pub struct TextureDesc {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u8,
    pub array_layers: u16,
    pub samples: u8,
    pub flags: TextureFlags,
}

pub struct Texture {
    pub id: TextureId,
    pub descriptor: TextureDesc,
    pub handle: u64,
    pub size: u64,
    pub gpu_memory: u64,
}

pub type TextureId = u64;

impl Texture {
    pub fn new(desc: TextureDesc) -> Self {
        let size = calc_texture_size(&desc);
        Self {
            id: 0,
            descriptor: desc,
            handle: 0,
            size,
            gpu_memory: size,
        }
    }

    pub fn width(&self) -> u32 { self.descriptor.width }
    pub fn height(&self) -> u32 { self.descriptor.height }
    pub fn format(&self) -> TextureFormat { self.descriptor.format }

    pub fn write(&mut self, data: &[u8], mip: u8, layer: u16) {
        // Upload texture data to GPU
    }

    pub fn read(&self, dst: &mut [u8], mip: u8, layer: u16) {
        // Readback texture data from GPU
    }
}

fn calc_texture_size(desc: &TextureDesc) -> u64 {
    let bpp = desc.format.bytes_per_pixel() as u64;
    let mut size = desc.width as u64 * desc.height as u64 * bpp;
    for _mip in 0..desc.mip_levels {
        size = size / 2;
    }
    size * desc.array_layers as u64
}
