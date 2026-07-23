//! BharatOS libhal audio — HDA codec
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct AudioFlags: u32 { const PLAYBACK = 1 << 0; const RECORD = 1 << 1; const LOOPBACK = 1 << 2; const MUTED = 1 << 3; }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioFormat { U8, S16LE, S24LE, S32LE, F32LE }

impl AudioFormat {
    pub fn bytes_per_sample(&self) -> u8 {
        match self { Self::U8 => 1, Self::S16LE => 2, Self::S24LE => 3, Self::S32LE | Self::F32LE => 4 }
    }
}

#[repr(C)]
pub struct AudioDevice {
    pub name: [u8; 64],
    pub ty: AudioDeviceType,
    pub sample_rates: [u32; 16],
    pub channels: [u8; 8],
    pub volume: f32,
    pub muted: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioDeviceType { Output, Input, Duplex, Virtual }

pub fn detect_devices() {
}

impl AudioDevice {
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn is_playing(&self) -> bool {
        !self.muted && self.volume > 0.0
    }

    pub fn supports_rate(&self, rate: u32) -> bool {
        self.sample_rates.contains(&rate)
    }
}
