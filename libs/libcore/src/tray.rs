//! BharatOS libcore tray icon system
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct TrayFlags: u32 {
        const VISIBLE      = 1 << 0;
        const BALLOON      = 1 << 1;
        const MENU         = 1 << 2;
    }
}

#[repr(C)]
pub struct TrayIcon {
    pub id: u64,
    pub flags: TrayFlags,
    pub icon_32: [u8; 32 * 32 * 4],
    pub icon_64: [u8; 64 * 64 * 4],
    pub tooltip: [u8; 128],
    pub title: [u8; 64],
    pub menu_items: Vec<TrayMenuItem>,
    pub click_callback: fn(u64),
    pub callback_data: u64,
}

#[repr(C)]
pub struct TrayMenuItem {
    pub id: u32,
    pub label: [u8; 64],
    pub kind: TrayMenuItemKind,
    pub enabled: bool,
    pub checked: bool,
    pub icon: [u8; 16],
    pub callback: fn(u32),
    pub submenu: Vec<TrayMenuItem>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TrayMenuItemKind {
    Normal,
    Checkbox,
    Radio,
    Separator,
    SubMenu,
}

pub struct TrayManager {
    pub icons: Vec<TrayIcon>,
    pub visible: bool,
    pub alignment: TrayAlignment,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TrayAlignment { Left, Center, Right }

impl TrayManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn add_icon(&mut self, icon: TrayIcon) -> u64 {
        self.icons.push(icon);
        (self.icons.len() - 1) as u64
    }

    pub fn remove_icon(&mut self, id: u64) {
        if (id as usize) < self.icons.len() {
            self.icons.remove(id as usize);
        }
    }

    pub fn show_balloon(&mut self, icon_id: u64, title: &str, body: &str) {
        let _ = (icon_id, title, body);
    }

    pub fn set_tooltip(&mut self, icon_id: u64, tooltip: &str) {
        if let Some(icon) = self.icons.get_mut(icon_id as usize) {
            let len = tooltip.len().min(127);
            icon.tooltip[..len].copy_from_slice(&tooltip.as_bytes()[..len]);
        }
    }
}
