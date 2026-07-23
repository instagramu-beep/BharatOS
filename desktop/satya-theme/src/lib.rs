//! BharatOS Satya Theme — Material-inspired dynamic themes
#![no_std]
#![allow(unused)]

pub mod theme;
pub mod colors;
pub mod icons;
pub mod fonts;
pub mod animations;
pub mod styles;

bitflags::bitflags! {
    pub struct ThemeFlags: u32 {
        const DARK = 1 << 0;
        const LIGHT = 1 << 1;
        const AUTO = 1 << 2;
        const HIGH_CONTRAST = 1 << 3;
        const REDUCED_MOTION = 1 << 4;
        const LARGE_TEXT = 1 << 5;
    }
}
