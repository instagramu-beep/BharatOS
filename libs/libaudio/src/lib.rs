//! BharatOS libaudio — HDA-compatible audio pipeline
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod hda;
pub mod mixer;
pub mod stream;
pub mod midi;
pub mod codec;
pub mod controls;

pub use stream::*;
pub use mixer::*;

bitflags::bitflags! {
    pub struct AudioFlags: u32 {
        const PLAYBACK = 1 << 0;
        const RECORD = 1 << 1;
        const LOOPBACK = 1 << 2;
        const MUTED = 1 << 3;
        const VIRTUAL = 1 << 4;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioFormat { U8, S16LE, S24LE, S32LE, F32LE }

impl AudioFormat {
    pub fn bytes_per_sample(&self) -> u8 {
        match self {
            Self::U8 => 1,
            Self::S16LE => 2,
            Self::S24LE => 3,
            Self::S32LE | Self::F32LE => 4,
        }
    }
}
