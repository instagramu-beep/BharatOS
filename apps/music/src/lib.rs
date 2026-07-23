//! BharatOS Music Player
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct MusicFlags: u32 {
        const PLAYING = 1 << 0;
        const PAUSED = 1 << 1;
        const REPEAT_OFF = 1 << 2;
        const REPEAT_ONE = 1 << 3;
        const REPEAT_ALL = 1 << 4;
        const SHUFFLE = 1 << 5;
        const CROSSFADE = 1 << 6;
        const EQ_ENABLED = 1 << 7;
        const VISUALIZER = 1 << 8;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioCodec { MP3, FLAC, AAC, OGG, WAV, ALAC, WMA, Opus, Unknown }

#[repr(C)]
pub struct Track {
    pub title: [u8; 256],
    pub artist: [u8; 128],
    pub album: [u8; 128],
    pub genre: [u8; 64],
    pub year: u16,
    pub track_number: u16,
    pub disc_number: u16,
    pub duration_ms: u32,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub codec: AudioCodec,
    pub path: [u8; 512],
    pub album_art: Option<&'static [u8]>,
    pub lyrics: Option<&'static [u8]>,
    pub rating: u8,
    pub play_count: u32,
    pub last_played: u128,
}

#[repr(C)]
pub struct Playlist {
    pub name: [u8; 128],
    pub tracks: Vec<Track>,
    pub current_index: usize,
    pub flags: MusicFlags,
    pub repeat: RepeatMode,
    pub shuffle: bool,
    pub volume: f32,
    pub eq: Equalizer,
    pub crossfade_ms: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RepeatMode { Off, One, All }

#[repr(C)]
pub struct Equalizer {
    pub preamp: f32,
    pub bands: [f32; 10],
    pub preset: EqPreset,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EqPreset { Flat, Rock, Pop, Jazz, Classical, Electronic, BassBoost, TrebleBoost, Vocal, Custom }

impl Playlist {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.tracks.remove(index);
        }
    }

    pub fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() { return None; }
        self.current_index = (self.current_index + 1) % self.tracks.len();
        self.tracks.get(self.current_index)
    }

    pub fn prev(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() { return None; }
        self.current_index = if self.current_index == 0 { self.tracks.len() - 1 } else { self.current_index - 1 };
        self.tracks.get(self.current_index)
    }

    pub fn current(&self) -> Option<&Track> {
        self.tracks.get(self.current_index)
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.5);
    }

    pub fn toggle_play(&mut self) {
        self.flags.toggle(MusicFlags::PLAYING);
    }

    pub fn seek(&mut self, position_ms: u32) {
        let _ = position_ms;
    }
}
