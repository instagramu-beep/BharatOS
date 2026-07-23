//! BharatOS libcore audio subsystem
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct AudioFlags: u32 {
        const PLAYBACK     = 1 << 0;
        const RECORDING    = 1 << 1;
        const LOOPBACK     = 1 << 2;
        const MUTED        = 1 << 3;
    }
}

pub enum AudioFormat {
    U8,
    S16LE,
    S24LE,
    S32LE,
    F32LE,
}

impl AudioFormat {
    pub fn bytes_per_sample(&self) -> u8 {
        match self { Self::U8 => 1, Self::S16LE => 2, Self::S24LE => 3, Self::S32LE | Self::F32LE => 4 }
    }
}

#[repr(C)]
pub struct AudioStream {
    pub format: AudioFormat,
    pub sample_rate: u32,
    pub channels: u8,
    pub flags: AudioFlags,
    pub volume: f32,
    pub buffer_size: u32,
    pub latency_ms: u32,
}

#[repr(C)]
pub struct AudioDevice {
    pub name: [u8; 64],
    pub ty: AudioDeviceType,
    pub sample_rates: [u32; 16],
    pub channels: [u8; 8],
    pub default_rate: u32,
    pub default_channels: u8,
}

#[derive(Clone, Copy)]
pub enum AudioDeviceType { Output, Input, Duplex, Virtual }

pub struct AudioManager {
    pub default_output: Option<u32>,
    pub default_input: Option<u32>,
    pub devices: Vec<AudioDevice>,
    pub mixer: MixerState,
}

#[derive(Clone, Copy)]
pub struct MixerState {
    pub master_volume: f32,
    pub bass: f32,
    pub treble: f32,
    pub balance: f32,
    pub spatial_audio: bool,
    pub noise_suppression: bool,
    pub echo_cancellation: bool,
    pub auto_gain: bool,
}

impl AudioManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.default_output = Some(0);
        self.default_input = Some(0);
        self.mixer = MixerState {
            master_volume: 1.0,
            bass: 0.0,
            treble: 0.0,
            balance: 0.0,
            spatial_audio: true,
            noise_suppression: true,
            echo_cancellation: true,
            auto_gain: true,
        };
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.mixer.master_volume = volume.clamp(0.0, 1.5);
    }

    pub fn toggle_mute(&mut self) {
        self.mixer.master_volume = if self.mixer.master_volume > 0.0 { 0.0 } else { 1.0 };
    }
}

pub struct AudioStreamHandle(pub u64);
