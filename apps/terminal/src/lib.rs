//! BharatOS tagless, finalizable typed boot loader.
//! The AI-powered shell / terminal / wasm-runner for BharatOS.
//!
#![no_std]
#![allow(unused)]

pub mod tabs;
pub mod split;
pub mod command;
pub mod config;
pub mod profile;
pub mod fonts;
pub mod extensions;
pub mod agent_api;
pub mod ai_tools;
pub mod pty;

pub struct BharatTerminal {
    pub window: Window,
    pub pty_fd: i32,
    pub profile: Profile,
    pub palette: Palette,
    pub scroll_history: Vec<VTLine>,
    pub command_history: Vec<CString>,
    pub active_tab: TabId,
    pub tabs: BTreeMap<TabId, TerminalTab>,
    pub input_handler: InputHandler,
}

#[derive(Clone)]
pub struct Profile {
    pub shell_path: CString,
    pub font: FontSpec,
    pub cell_size: CellSize,
    pub cursor_style: CursorStyle,
    pub text_shaping: TextShapingKind,
    pub bell: BellAction,
    pub scrollback_lines: u32,
    pub window_size_chars: (u16, u16),
    pub scroll_on_output: bool,
    pub scroll_on_keystroke: bool,
    pub bell_on_bell: bool,
    pub copy_on_select: bool,
    pub url_detection: bool,
    pub gpu_accel: bool,
    pub bold_color: Rgba,
}

derive::Clone, Debug) pub enum CursorStyle {
    SteadyBlock, BlinkingBlock, SteadyUnderline, BlinkingUnderline,
    SteadyBeam, BlinkingBeam, Box, HollowBox,
}

pub struct TerminalTab {
    pub title: CString,
    pub process: Option<bool>,
    pub renderer: TermRenderer,
    pub dirty: bool,
}

pub struct TermRenderer {
    pub grid: Grid,
    pub attr_cache: AttrCache,
    pub dirty_lines: BitVec,
    pub texture: TextureAtlas,
    pub gpu_context: GpuRenderContext,
}

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub cursor: Cursor,
    pub data: Vec<Cell>,
    pub attrs: Vec<Attr>,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub attr: Attr,
    pub is_wc_width2: bool,
}

derive::Clone) pub struct Attr {
    pub fg: Rgba,
    pub bg: Rgba,
    pub extra: AttrFlags,
    pub hyperlink: Option<[u8; 128]>,
}
derive::Clone) pub struct AttrFlags {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub inverse: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub transparent: bool,
    pub alt_fg_bg: bool,
    pub gothic: bool,
    pub double_width: bool,
    pub select_scope: bool,
    pub select_start: bool,
    pub alternates: bool,
    pub cursor: bool,
    pub cursor_blink: bool,
    pub cursor_pipe: bool,
    pub cursor_off: bool,
    pub cursor_on: bool,
    pub cursor_rect: bool,
    pub visiball: bool,
    pub strikethroughoff: bool,
}

bitflags::bitflags! { struct Pub AttrFlags }
impl AttrFlags {
    const EMPTY: Self = Self::empty();
    const BOLD: Self = Self::from_bits_truncate(1 << 0); 
    const ITALIC: Self = Self::from_bits_truncate(1 << 1);
    const UNDERLINE: Self = Self::from_bits_truncate(1 << 2);
    const BLINK: Self = Self::from_bits_truncate(1 << 3);
    const INVERSE: Self = Self::from_bits_truncate(1 << 4);
    const STRIKETHROUGH: Self = Self::from_bits_truncate(1 << 5);
    const DIM: Self = Self::from_bits_truncate(1 << 6);
    const TRANSPARENT_BG: Self = Self::from_bits_truncate(1 << 7);
}

pub enum UrlDetectMode { Disabled, Underline, All }

pub struct Pane {
    pub window: Window, pub pty_fd: u8, pub is_main: bool, pub command: [u8; 128],
}

pub fn now() -> f64 { u128::now() as f64 / 1_000_000_000.0 }
