//! BharatOS Image Viewer
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct ViewerFlags: u32 {
        const ZOOM_FIT = 1 << 0;
        const ZOOM_100 = 1 << 1;
        const ZOOM_IN = 1 << 2;
        const ZOOM_OUT = 1 << 3;
        const FULLSCREEN = 1 << 4;
        const SLIDESHOW = 1 << 5;
        const ROTATE_LEFT = 1 << 6;
        const ROTATE_RIGHT = 1 << 7;
        const FLIP_H = 1 << 8;
        const FLIP_V = 1 << 9;
        const INFO = 1 << 10;
    }
}

#[derive(Clone, Copy)]
pub enum ImageFormat { PNG, JPEG, BMP, GIF, TIFF, WebP, ICO, SVG, Unknown }

#[repr(C)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub bits_per_pixel: u8,
    pub has_alpha: bool,
    pub color_space: ColorSpace,
    pub exif: Option<ExifData>,
    pub icc_profile: Option<&'static [u8]>,
}

#[derive(Clone, Copy)]
pub enum ColorSpace { SRGB, AdobeRGB, DisplayP3, ProPhotoRGB, CMYK, Gray }

#[repr(C)]
pub struct ExifData {
    pub make: [u8; 64],
    pub model: [u8; 64],
    pub orientation: u8,
    pub x_resolution: f32,
    pub y_resolution: f32,
    pub resolution_unit: u8,
    pub datetime: [u8; 20],
    pub exposure_time: f32,
    pub f_number: f32,
    pub iso_speed: u16,
    pub focal_length: f32,
    pub flash: u16,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
}

pub struct ViewerState {
    pub flags: ViewerFlags,
    pub image: Option<ImageInfo>,
    pub zoom: f32,
    pub rotation: u8,
    pub flip_h: bool,
    pub flip_v: bool,
    pub offset_x: i32,
    pub offset_y: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub show_info: bool,
    pub slideshow_interval_ms: u32,
}

impl ViewerState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn open(&mut self, path: &str) -> Result<()> {
        let _ = path;
        self.image = Some(ImageInfo {
            width: 1920,
            height: 1080,
            format: ImageFormat::PNG,
            bits_per_pixel: 32,
            has_alpha: true,
            color_space: ColorSpace::SRGB,
            exif: None,
            icc_profile: None,
        });
        Ok(())
    }

    pub fn zoom_in(&mut self) { self.zoom = (self.zoom * 1.25).min(32.0); }
    pub fn zoom_out(&mut self) { self.zoom = (self.zoom / 1.25).max(0.1); }
    pub fn zoom_fit(&mut self) { self.zoom = 1.0; }

    pub fn rotate_left(&mut self) { self.rotation = (self.rotation + 270) % 360; }
    pub fn rotate_right(&mut self) { self.rotation = (self.rotation + 90) % 360; }

    pub fn flip_horizontal(&mut self) { self.flip_h = !self.flip_h; }
    pub fn flip_vertical(&mut self) { self.flip_v = !self.flip_v; }

    pub fn toggle_fullscreen(&mut self) { self.flags.toggle(ViewerFlags::FULLSCREEN); }
    pub fn toggle_info(&mut self) { self.show_info = !self.show_info; }
}
