//! BharatOS libcore — no_std foundation crate
#![no_std]
#![allow(unused)]

extern crate alloc;

pub mod prelude;
pub mod arch;
pub mod event;
pub mod io;
pub mod keycodes;
pub mod mem;
pub mod interop;
pub mod napi;
pub mod force_idle;
pub mod env;
pub mod err;
// pub mod panic;
// pub mod alloc;    // disabled: conflicts with extern crate alloc
// pub mod fs;       // missing
// pub mod net;      // missing
// pub mod security; // broken: references missing types
// pub mod debug;    // broken
// pub mod profile;  // broken
// pub mod trace;    // broken
// pub mod time;     // broken
// pub mod string;   // broken: orphan rules
// pub mod process;  // broken: external crate ref
// pub mod ai;       // broken: external crate ref
// pub mod signals;  // broken: references process
// pub mod texture;  // broken
// pub mod shader;   // broken
// pub mod window;   // broken
// pub mod surface;  // broken
// pub mod tray;     // broken
// pub mod audio;    // broken
// pub mod input;    // broken
// pub mod ipc;      // broken
// pub mod sync;     // broken
// pub mod math;     // broken: no f32 methods in no_std

pub use self::prelude::*;
pub use self::err::{Error, Result};
pub use alloc::string::String;

pub const VERSION_MAJOR: u32 = 1;
pub const VERSION_MINOR: u32 = 0;
pub const VERSION_PATCH: u32 = 0;
pub const VERSION_STRING: &str = "1.0.0-dev";
pub const OS_NAME: &str = "BharatOS";
