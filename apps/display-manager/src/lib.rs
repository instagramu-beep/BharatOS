//! BharatOS Display Manager (bharat-dm) — compositor-background user login
//! with Sans-serif (Noto Sans) and Tyr font (virtual keyboard, Kannada support).
//! Provides:
//!   - greeter session — dark login form (username, password, biometric, NFC)
//!   - session pivot — systemd-replacement (bus socket -> /run/current-system)
//!   - xdg-desktop-portal parity
//!   - greeten-time user avatar
//!   - power-off / sleep / restart buttons
//!   - accessibility zoom
//!   - Brightness control

#![no_std]

use crate::prelude::*;

pub mod greeter;
pub mod session;
pub mod plasma_noaccel;
pub mod portal;

pub struct BharatDM {
    pub compositor_handle: CompositorHandle,
    pub greeter_renderer: greeter::GreeterRenderer,
    pub current_session: Option<UserSession>,
    pub policy: dm::Policy,
}

pub struct UserSession {
    pub uid: u32,
    pub user: [u8; 32],          // login name
    pub home_dir: [u8; 128],
    pub session_type: SessionType,
    pub theme_id: u32,
    pub autostart_entries: Vec<[u8; 64]>,
    pub saved_window_positions: Vec<WindowGeometry>,
    pub biometric_pref: BiometricType,
    pub env_overrides: BTreeMap<Hash256, Value>,
}

#[repr(u8)]
pub enum SessionType {
    SatyaDesktop = 0,
    BharatX11  = 1,
    TTY       = 2,
    XdgWM     = 3,
}

#[repr(u8)]
pub enum BiometricType {
    None = 0, Password = 1, Fingerprint = 2, FaceID = 3, PIN = 4, NFC = 5, Union = 6,
}

#[derive(Clone, Copy)]
pub struct DisplayPolicy {
    pub disable_lock: bool,
    pub lock_on_suspend: bool,
    pub auto_login: Option<u32>, // uid or None
    pub timeout_secs: u32,
    pub blank_screen_timeout: u32,
    pub role: AccountKind,
    pub users_avatars_url: Vec<Url>,
}

pub struct GreeterRenderer {
    pub surface: Surface,
    pub greeter_username: CString,
    pub greeter_state: crate::sih::Pane,
    pub avata_model_url: Option<crate::surface::gen_model::AvatarModel>,
}
