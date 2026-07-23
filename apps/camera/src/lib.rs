//! BharatOS Camera App
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct CameraFlags: u32 {
        const STREAMING = 1 << 0;
        const RECORDING = 1 << 1;
        const PAUSED = 1 << 2;
        const FLASH = 1 << 3;
        const HDR = 1 << 4;
        const NIGHT_MODE = 1 << 5;
        const PORTRAIT = 1 << 6;
        const TIME_LAPSE = 1 << 7;
        const SLOW_MO = 1 << 8;
        const GRID = 1 << 9;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CameraFacing { Back, Front, External, Unknown }

#[derive(Clone, Copy, PartialEq)]
pub enum CaptureMode { Photo, Video, Panorama, Portrait, Night, TimeLapse, SlowMotion, Burst }

#[repr(C)]
pub struct CameraDevice {
    pub id: u32,
    pub name: [u8; 64],
    pub facing: CameraFacing,
    pub resolutions: [Resolution; 16],
    pub resolution_count: u8,
    pub supported_modes: [CaptureMode; 8],
    pub mode_count: u8,
    pub has_flash: bool,
    pub has_autofocus: bool,
    pub has_ois: bool,
    pub max_digital_zoom: f32,
    pub sensor_size: SensorSize,
}

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub format: PixelFormat,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PixelFormat { RGB888, RGBA8888, YUV420, NV12, NV21, YUYV, RAW10, RAW12 }

#[derive(Clone, Copy)]
pub struct SensorSize { pub width_mm: f32, pub height_mm: f32, pub crop_factor: f32 }

pub struct CameraState {
    pub flags: CameraFlags,
    pub device: Option<CameraDevice>,
    pub mode: CaptureMode,
    pub resolution: Resolution,
    pub exposure_ms: f32,
    pub iso: u16,
    pub white_balance: WhiteBalance,
    pub focus_distance: f32,
    pub zoom: f32,
    pub flash_mode: FlashMode,
    pub timer: u32,
    pub filters: Vec<ImageFilter>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WhiteBalance { Auto, Daylight, Cloudy, Tungsten, Fluorescent, Flash, Custom }

#[derive(Clone, Copy, PartialEq)]
pub enum FlashMode { Off, Auto, On, Torch, RedEye }

#[derive(Clone, Copy)]
pub struct ImageFilter {
    pub name: [u8; 32],
    pub intensity: f32,
    pub ty: FilterType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FilterType { None, Grayscale, Sepia, Invert, Blur, Sharpen, Vignette, Contrast, Saturation, Warm, Cool, Vintage }

impl CameraState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.flags = CameraFlags::empty();
        self.mode = CaptureMode::Photo;
        self.resolution = Resolution { width: 1920, height: 1080, fps: 30, format: PixelFormat::RGB888 };
        self.exposure_ms = 10.0;
        self.iso = 100;
        self.white_balance = WhiteBalance::Auto;
        self.focus_distance = 0.0;
        self.zoom = 1.0;
        self.flash_mode = FlashMode::Auto;
        self.timer = 0;
    }

    pub fn capture_photo(&mut self) -> Result<CapturedPhoto> {
        self.flags.insert(CameraFlags::STREAMING);
        Ok(CapturedPhoto {
            width: self.resolution.width,
            height: self.resolution.height,
            format: self.resolution.format,
            exposure_ms: self.exposure_ms,
            iso: self.iso,
            timestamp: crate::time::timestamp(),
        })
    }

    pub fn start_video(&mut self) { self.flags.insert(CameraFlags::RECORDING); }
    pub fn stop_video(&mut self) { self.flags.remove(CameraFlags::RECORDING); }

    pub fn set_exposure(&mut self, ms: f32) { self.exposure_ms = ms.max(0.1).min(1000.0); }
    pub fn set_iso(&mut self, iso: u16) { self.iso = iso.min(6400); }
    pub fn set_zoom(&mut self, z: f32) { self.zoom = z.max(1.0).min(10.0); }
}

#[derive(Clone, Copy)]
pub struct CapturedPhoto {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub exposure_ms: f32,
    pub iso: u16,
    pub timestamp: u128,
}
