//! BharatOS libcore font management
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct FontFlags: u32 {
        const HINTING     = 1 << 0;
        const SUBPIXEL    = 1 << 1;
        const EMBEDDED_BMP = 1 << 2;
        const MONO        = 1 << 3;
        const COLOR       = 1 << 4;
        const VARIABLE    = 1 << 5;
    }
}

#[derive(Clone, Copy)]
pub struct FontDesc {
    pub family: [u8; 64],
    pub style: u16,
    pub weight: u16,
    pub stretch: u16,
    pub flags: FontFlags,
    pub size: f32,
    pub dpi: u32,
    pub script: u32,
}

#[repr(C)]
pub struct GlyphMetrics {
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance: u16,
}

#[repr(C)]
pub struct FontMetrics {
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
    pub height: u16,
    pub max_advance: u16,
}
