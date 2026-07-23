//! BharatOS Notes App
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct NoteFlags: u32 {
        const MODIFIED = 1 << 0;
        const PINNED = 1 << 1;
        const SHARED = 1 << 2;
        const LOCKED = 1 << 3;
        const ENCRYPTED = 1 << 4;
        const HAS_ATTACHMENTS = 1 << 5;
        const HAS_CHECKLIST = 1 << 6;
        const HAS_HANDWRITING = 1 << 7;
        const DELETED = 1 << 8;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum NoteKind { Text, Checklist, Handwriting, Sketch, Mixed, Audio, Image }

#[derive(Clone, Copy, PartialEq)]
pub enum ChecklistItemStatus { Todo, Done, Cancelled }

#[repr(C)]
pub struct Note {
    pub id: u64,
    pub title: [u8; 256],
    pub content: Vec<u8>,
    pub kind: NoteKind,
    pub flags: NoteFlags,
    pub created_ms: u128,
    pub modified_ms: u128,
    pub accessed_ms: u128,
    pub tags: [u8; 256],
    pub color: u32,
    pub checklist: Vec<ChecklistItem>,
    pub attachments: Vec<Attachment>,
    pub notebook_id: u32,
}

#[repr(C)]
pub struct ChecklistItem {
    pub text: [u8; 256],
    pub status: ChecklistItemStatus,
    pub indent: u8,
    pub due_ms: u64,
    pub priority: u8,
}

#[repr(C)]
pub struct Attachment {
    pub name: [u8; 128],
    pub mime_type: [u8; 64],
    pub size: u64,
    pub data: Option<&'static [u8]>,
    pub thumbnail: Option<&'static [u8]>,
}

#[repr(C)]
pub struct Notebook {
    pub id: u32,
    pub name: [u8; 128],
    pub parent_id: u32,
    pub color: u32,
    pub icon: [u8; 32],
    pub note_count: u32,
    pub created_ms: u128,
    pub modified_ms: u128,
}

pub struct NotesState {
    pub notes: Vec<Note>,
    pub notebooks: Vec<Notebook>,
    pub current_note: Option<u64>,
    pub selected_notebook: u32,
    pub sort_by: SortBy,
    pub filter_tags: Vec<u8>,
    pub search_query: [u8; 128],
    pub view_mode: ViewMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortBy { Modified, Created, Title, Color, Size }

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode { List, Grid, Split, Focus }

impl NotesState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.sort_by = SortBy::Modified;
        self.view_mode = ViewMode::List;
        self.selected_notebook = 0;
        self.notes = Vec::new();
        self.notebooks = vec![Notebook {
            id: 0,
            name: *b"My Notes\0",
            parent_id: 0,
            color: 0xFF4285F4,
            icon: *b"notes\0",
            note_count: 0,
            created_ms: crate::time::timestamp(),
            modified_ms: crate::time::timestamp(),
        }];
    }

    pub fn create_note(&mut self, kind: NoteKind) -> &mut Note {
        let id = crate::time::timestamp() as u64;
        let note = Note {
            id,
            title: [0; 256],
            content: Vec::new(),
            kind,
            flags: NoteFlags::MODIFIED,
            created_ms: crate::time::timestamp(),
            modified_ms: crate::time::timestamp(),
            accessed_ms: crate::time::timestamp(),
            tags: [0; 256],
            color: 0xFFFFFFFF,
            checklist: Vec::new(),
            attachments: Vec::new(),
            notebook_id: self.selected_notebook,
        };
        self.notes.push(note);
        self.notes.last_mut().unwrap()
    }

    pub fn delete_note(&mut self, id: u64) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.flags.insert(NoteFlags::DELETED);
        }
    }

    pub fn pin_note(&mut self, id: u64) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.flags.toggle(NoteFlags::PINNED);
        }
    }

    pub fn set_color(&mut self, id: u64, color: u32) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.color = color;
        }
    }

    pub fn add_tag(&mut self, id: u64, tag: &str) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            let _ = tag;
        }
    }

    pub fn search(&self, query: &str) -> Vec<&Note> {
        self.notes.iter().filter(|n| {
            let title = unsafe { core::str::from_utf8_unchecked(&n.title) };
            title.contains(query)
        }).collect()
    }
}
