//! BharatOS libsurface compositor integration
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::libcore::{Window, Surface};

pub struct CompositorSurface {
    pub surface: Option<&'static mut Surface>,
    pub layer: Layer,
    pub cache_hash: u64,
    pub scale: f32,
    pub damage: [Rect; 8],
    pub damage_count: u8,
    pub last_frame: u128,
}

#[repr(C)]
pub struct Layer {
    pub ty: LayerType,
    pub flags: LayerFlags,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub source_rect: Rect,
    pub dest_rect: Rect,
    pub corner_radius: f32,
    pub transform: u32,
    pub shader: u32,
}

bitflags::bitflags! {
    pub struct LayerFlags: u32 {
        const BLOOM = 1 << 0;
        const BLUR = 1 << 1;
        const SHADOW = 1 << 2;
        const REFLECTION = 1 << 3;
        const ROUNDED = 1 << 4;
    }
}

#[derive(Clone, Copy)]
pub enum LayerType {
    Background = 0,
    Window = 1,
    Overlay = 2,
    Cursor = 3,
    Notification = 4,
    Shell = 5,
    Panel = 6,
}

#[derive(Clone, Copy)]
pub enum BlendMode { Over, Add, Multiply, Screen, Overlay }

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self { Self { x, y, width: w, height: h } }
    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width as i32 &&
        self.x + self.width as i32 > other.x &&
        self.y < other.y + other.height as i32 &&
        self.y + self.height as i32 > other.y
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width as i32 &&
        py >= self.y && py < self.y + self.height as i32
    }
}
