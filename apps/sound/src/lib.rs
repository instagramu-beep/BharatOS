//! BharatOS Sound Settings
#![no_std]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct AudioStreamFlags: u32 {
        const PLAYBACK = 1 << 0;
        const RECORD   = 1 << 1;
        const LOOPBACK = 1 << 2;
    }
}

#[repr(C)]
pub struct StreamInfo {
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub frame_size: u16,
    pub flags: AudioStreamFlags,
    pub volume: f32,
    pub muted: bool,
    pub buffer_size: u32,
    pub latency_ms: u32,
}

#[derive(Clone, Copy)]
pub enum AudioDeviceKind { Output, Input, Duplex, Virtual }

#[repr(C)]
pub struct AudioDevice {
    pub name: [u8; 64],
    pub kind: AudioDeviceKind,
    pub sample_rates: [u32; 16],
    pub channels: [u8; 8],
    pub default_rate: u32,
    pub default_channel: u8,
    pub volume: f32,
    pub muted: bool,
    pub exclusive: bool,
}

pub struct SoundManager {
    pub default_output: Option<u32>,
    pub default_input: Option<u32>,
    pub devices: Vec<AudioDevice>,
    pub mixer: MixerNode,
    pub stream: [Option<StreamInfo>; 16],
}

impl SoundManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.default_output = Some(0);
        self.default_input = Some(0);
    }

    pub fn set_volume(&mut self, device: u32, vol: f32) {
        if let Some(d) = self.devices.get_mut(device as usize) {
            d.volume = vol.clamp(0.0, 1.5);
        }
    }

    pub fn toggle_mute(&mut self, device: u32) {
        if let Some(d) = self.devices.get_mut(device as usize) {
            d.muted = !d.muted;
        }
    }

    pub fn select_output(&mut self, device: u32) {
        self.default_output = Some(device);
    }

    pub fn set_stream_volume(&mut self, idx: usize, vol: f32) {
        if let Some(s) = self.stream.get_mut(idx) {
            if let Some(stream) = s.as_mut() {
                stream.volume = vol.clamp(0.0, 1.0);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct MixerNode {
    pub bass: f32,
    pub treble: f32,
    pub balance: f32,
    pub fader: f32,
    pub eq_preset: u8,
    pub spatial_audio: bool,
    pub noise_suppression: bool,
    pub echo_cancellation: bool,
    pub auto_gain: bool,
}
