//! BharatOS libcore text shaping (HarfBuzz-compatible)
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct Shaper;
pub struct Font;
pub struct Buffer;

pub struct Feature {
    pub tag: u32,
    pub value: u32,
    pub start: u32,
    pub end: u32,
}

pub struct GlyphInfo {
    pub codepoint: u32,
    pub cluster: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub flags: u32,
}

pub struct GlyphPosition {
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub g1: u16,
    pub g2: u16,
}

pub struct TextCluster {
    pub byte_start: u32,
    pub byte_end: u32,
    pub glyph_start: u16,
    pub glyph_end: u16,
}

impl Shaper {
    pub fn shape(font: &Font, buffer: &mut Buffer, features: &[Feature]) -> Result<Vec<GlyphInfo>> {
        Ok(Vec::new())
    }
}

impl Font {
    pub fn from_bytes(data: &[u8], index: u32) -> Result<Self> {
        Ok(Self { _data: data, _idx: index })
    }
    pub fn glyph_count(&self) -> u32 { 0 }
    pub fn has_glyph(&self, codepoint: u32) -> bool { true }
    pub fn nom_scale(&self) -> f32 { 1.0 }
    pub fn em_scale(&self) -> f32 { 1.0 }
}

impl Buffer {
    pub fn new() -> Self { Self { _buf: Vec::new(), _glyph_count: 0, _cluster_count: 0 } }
    pub fn add_utf8(&mut self, text: &str) { self._buf.extend_from_slice(text.as_bytes()); }
    pub fn guess_segment_properties(&mut self) {
        let _ = self;
    }
    pub fn glyph_count(&self) -> u32 { self._glyph_count }
    pub fn cluster_level(&self) -> u8 { 0 }
}
