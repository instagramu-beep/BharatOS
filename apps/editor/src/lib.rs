//! BharatOS Text Editor — Saty Editor (Material-inspired, GPU-rendered text)
#![no_std]

pub mod model;
pub mod view;
pub mod syntax;

bitflags::bitflags! {
    pub struct EditorFlags: u32 {
        const READ_ONLY     = 1 << 0;
        const MODIFIED      = 1 << 1;
        const WORD_WRAP     = 1 << 2;
        const SYNTAX_HL     = 1 << 3;
        const AUTO_INDENT   = 1 << 4;
        const SHOW_LINE_NUM = 1 << 5;
        const SHOW_RULER    = 1 << 6;
        const MINIMAP       = 1 << 7;
        const AI_SUGGEST    = 1 << 8;
        const SPELL_CHECK   = 1 << 9;
    }
}

#[repr(C)]
pub struct EditorState {
    pub flags: EditorFlags,
    pub cursor_line: u32,
    pub cursor_col: u32,
    pub scroll_y: u32,
    pub scroll_x: u32,
    pub selection_start: (u32, u32),
    pub selection_end: (u32, u32),
    pub tab_size: u8,
    pub indent_style: IndentStyle,
    pub font_size: u8,
    pub line_ending: LineEnding,
    pub encoding: Encoding,
    pub theme: u32,
    pub undo_stack: Vec<EditOp>,
    pub redo_stack: Vec<EditOp>,
}

#[derive(Clone, Copy)]
pub enum IndentStyle { Spaces, Tabs, Mixed }

#[derive(Clone, Copy)]
pub enum LineEnding { LF, CRLF, CR }

#[derive(Clone, Copy)]
pub enum Encoding { UTF8, UTF16, Latin1, ASCII }

#[repr(C)]
pub struct EditOp {
    pub kind: EditKind,
    pub line: u32,
    pub col: u32,
    pub old_text: [u8; 256],
    pub new_text: [u8; 256],
    pub old_len: u16,
    pub new_len: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum EditKind {
    InsertChar,
    InsertLine,
    DeleteChar,
    DeleteLine,
    Replace,
    Indent,
    Format,
}

impl EditorState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(path: Option<&str>) -> Self {
        let mut state = Self::new();
        state.flags.insert(EditorFlags::SYNTAX_HL);
        state.flags.insert(EditorFlags::SHOW_LINE_NUM);
        state.flags.insert(EditorFlags::AUTO_INDENT);
        state.flags.insert(EditorFlags::WORD_WRAP);
        state.tab_size = 4;
        state.indent_style = IndentStyle::Spaces;
        state.line_ending = LineEnding::LF;
        state.encoding = Encoding::UTF8;
        state.font_size = 14;
        state
    }

    pub fn insert_char(&mut self, c: char) {
        self.flags.insert(EditorFlags::MODIFIED);
        let op = EditOp {
            kind: EditKind::InsertChar,
            line: self.cursor_line,
            col: self.cursor_col,
            old_text: [0; 256],
            new_text: [c as u8; 1],
            old_len: 0,
            new_len: 1,
        };
        self.undo_stack.push(op);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            self.redo_stack.push(op);
        }
    }

    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            self.undo_stack.push(op);
        }
    }

    pub fn is_modified(&self) -> bool {
        self.flags.contains(EditorFlags::MODIFIED)
    }

    pub fn save(&mut self) -> Result<()> {
        self.flags.remove(EditorFlags::MODIFIED);
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        if self.is_modified() { return Err(crate::err::Error::IoError); }
        Ok(())
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
