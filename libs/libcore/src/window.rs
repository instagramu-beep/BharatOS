//! BharatOS window management
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::math::Vec2;

bitflags::bitflags! {
    pub struct WindowFlags: u32 {
        const DECORATED    = 1 << 0;
        const RESIZABLE    = 1 << 1;
        const MAXIMIZED    = 1 << 2;
        const MINIMIZED    = 1 << 3;
        const FULLSCREEN   = 1 << 4;
        const ALWAYS_ON_TOP = 1 << 5;
        const TRANSPARENT  = 1 << 6;
        const BLUR_BEHIND  = 1 << 7;
        const ARGB         = 1 << 8;
        const NO_TASKBAR   = 1 << 9;
        const POPUP        = 1 << 10;
        const MODAL        = 1 << 11;
        const FOCUSED      = 1 << 12;
    }
}

#[derive(Clone, Copy)]
pub enum WindowKind {
    Normal,
    Dialog,
    Popup,
    Tooltip,
    Splash,
    Utility,
    Desktop,
}

#[repr(C)]
pub struct WindowDesc {
    pub title: [u8; 128],
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub flags: WindowFlags,
    pub kind: WindowKind,
    pub position: Vec2,
    pub opacity: f32,
    pub theme: u32,
}

#[repr(C)]
pub struct WindowState {
    pub id: WindowId,
    pub desc: WindowDesc,
    pub surface: SurfaceHandle,
    pub visible: bool,
    pub focused: bool,
    pub active: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub position: Vec2,
    pub size: Vec2,
    pub opacity: f32,
    pub client_rect: (i32, i32, u32, u32),
    pub frame_rect: (i32, i32, u32, u32),
    pub last_frame_time: u128,
    pub input_focus: bool,
    pub always_on_top: bool,
}

pub type WindowId = u64;
pub type SurfaceHandle = u64;
