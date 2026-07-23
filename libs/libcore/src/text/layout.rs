//! BharatOS libcore text layout engine
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct TextLayout {
    pub width: u32,
    pub height: u32,
    pub lines: Vec<TextLine>,
    pub cursor: Option<CursorPos>,
    pub selection: Option<Selection>,
}

pub struct TextLine {
    pub y: u32,
    pub height: u32,
    pub glyphs: Vec<GlyphRef>,
    pub width: u32,
}

pub struct GlyphRef {
    pub font: u32,
    pub glyph: u32,
    pub x: u32,
    pub y: u32,
    pub color: u32,
    pub size: u8,
}

#[derive(Clone, Copy)]
pub struct CursorPos {
    pub line: u32,
    pub col: u32,
    pub x: u32,
    pub y: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct Selection {
    pub start: CursorPos,
    pub end: CursorPos,
    pub text: [u8; 256],
}

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn layout(text: &str, font: &FontDesc, max_width: u32) -> TextLayout {
        let mut lines = Vec::new();
        let mut y = 0u32;
        let mut x = 0u32;
        let mut line_glyphs = Vec::new();

        for c in text.chars() {
            if c == '\n' {
                lines.push(TextLine {
                    y, height: font.size as u32,
                    glyphs: core::mem::replace(&mut line_glyphs, Vec::new()),
                    width: x,
                });
                y += font.size as u32;
                x = 0;
                continue;
            }
            line_glyphs.push(GlyphRef {
                font: 0,
                glyph: c as u32,
                x, y,
                color: 0xFFFFFFFF,
                size: font.size as u8,
            });
            x += (font.size * 0.5) as u32;
        }

        if !line_glyphs.is_empty() {
            lines.push(TextLine {
                y, height: font.size as u32,
                glyphs: line_glyphs,
                width: x,
            });
        }

        TextLayout {
            width: max_width,
            height: y + font.size as u32,
            lines,
            cursor: None,
            selection: None,
        }
    }
}
