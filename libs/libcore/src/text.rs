//! BharatOS libcore text shaping and rendering abstractions
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::math;

pub mod shaping;
pub mod font;
pub mod layout;

pub use shaping::*;
pub use font::*;
pub use layout::*;

#[derive(Clone, Copy)]
pub struct Glyph {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub advance: f32,
    pub uv: [f32; 4],
    pub color: u32,
    pub flags: GlyphFlags,
}

bitflags::bitflags! {
    pub struct GlyphFlags: u32 {
        const BOLD = 1 << 0;
        const ITALIC = 1 << 1;
        const UNDERLINE = 1 << 2;
        const STRIKETHROUGH = 1 << 3;
        const COLOR = 1 << 4;
        const GRADIENT = 1 << 5;
        const OUTLINE = 1 << 6;
        const SHADOW = 1 << 7;
        const SUBPIXEL = 1 << 8;
        const KERNING = 1 << 9;
        const LIGATURES = 1 << 10;
        const DIACRITICS = 1 << 11;
    }
}

#[repr(C)]
pub struct TextRun {
    pub text: String,
    pub font: FontId,
    pub size: f32,
    pub color: u32,
    pub flags: GlyphFlags,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
pub struct Paragraph {
    pub runs: Vec<TextRun>,
    pub alignment: TextAlign,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub word_spacing: f32,
    pub indent: f32,
    pub max_width: f32,
}

#[derive(Clone, Copy)]
pub enum TextAlign { Left, Right, Center, Justify }

#[repr(C)]
pub struct FontFace {
    pub id: FontId,
    pub name: [u8; 32],
    pub path: [u8; 128],
    pub weight: u16,
    pub style: FontStyle,
    pub size: f32,
    pub scale: f32,
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub glyph_count: u16,
    pub data: Option<&'static [u8]>,
}

#[derive(Clone, Copy)]
pub enum FontStyle { Normal, Italic, Oblique }

pub type FontId = u32;

static mut FONT_REGISTRY: Vec<FontFace> = Vec::new();

pub struct TextShaper;

impl TextShaper {
    pub fn shape(text: &str, font: &FontFace) -> Result<Vec<Glyph>> {
        let mut glyphs = Vec::new();
        for (i, c) in text.chars().enumerate() {
            let g = Glyph {
                id: c as u32,
                x: i as f32 * font.size * 0.5,
                y: font.ascent,
                width: font.size * 0.5,
                height: font.size,
                advance: font.size * 0.5,
                uv: [0.0; 4],
                color: 0xFFFFFFFF,
                flags: GlyphFlags::empty(),
            };
            glyphs.push(g);
        }
        Ok(glyphs)
    }
}
