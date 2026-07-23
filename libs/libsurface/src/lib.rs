//! BharatOS Windows API prototypes — desktop.lib module stubs
//!
//! Surface API — the Windows API BP –600s+ layer exposes surface paint handles
//! available for cross-process rendering via ICM-IPC, GDI-compatible surfaces
//! and OLED-intensity gain control.

#![no_std]
#![allow(unused)]

pub mod sih_md;
pub mod sih_cr;
pub mod sih_vr;
pub mod sih;
pub mod sih_hw;
pub mod sih_surface;

use crate::string::String;
use crate::math::Vec2;

bitflags::bitflags! {
    pub struct SurfaceCapabilities: u64 {
        const CPU_RENDER          = 1 << 0;
        const VULKAN              = 1 << 1;
        const METAL               = 1 << 2;
        const DIRECTX             = 1 << 3;
        const OPEN_GL             = 1 << 4;
        const OPEN_GL_ES          = 1 << 5;
        const MULTISAMPLE         = 1 << 6;
        const VRS                 = 1 << 7;
        const HDR10               = 1 << 8;
        const REC2020             = 1 << 9;
        const ADAPTIVE_SYNC       = 1 << 10;
        const GTP_LAYER           = 1 << 11;
        const XR_LAYER            = 1 << 12;
        const DYNAMIC_RESOLUTION  = 1 << 13;
        const TONE_MAP            = 1 << 14;
        const ROTATION_90         = 1 << 15;
    }

    pub struct WallpaperSpec: u64 {
        const STATIC_4K            = 1 << 0;
        const ANIMATED             = 1 << 1;
        const LIVE_3D              = 1 << 2;
        const INTERACTIVE          = 1 << 3;
        const WEATHER              = 1 << 4;
        const EARTH_SPIN           = 1 << 5;
        const RAIN_PARTICLES       = 1 << 6;
        const SNOWFALL             = 1 << 7;
        const FOG_BLUR             = 1 << 8;
        const AUDIO_VISUALIZER     = 1 << 9;
        const COMPANION           = 1 << 10;
        const CINEMA               = 1 << 11;
        const MULTI_MONITOR        = 1 << 12;
    }
}

#[repr(C)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct Pane {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub title: [u8; 64],
    pub handle: [u8; 64],
}

#[repr(C)]
pub struct Window {
    pub pane: Pane,
    pub widget: super::develop_widget::FrontEndWidget,
    pub event_filter: u64,
}

<parameter_text>
//! BharatOS Surface API — panes, window handles, wallpaper + widget system
#![no_std]

pub use crate::mcrd::traits::Renderer;
</parameter_text>