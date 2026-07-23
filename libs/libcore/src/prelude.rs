//! BharatOS libcore prelude — consistent imports across kernel + desktop
#![no_std]

pub use alloc::vec::Vec;
pub use alloc::boxed::Box;
pub use alloc::rc::Rc;
pub use alloc::sync::Arc;
pub use alloc::string::String;
pub use alloc::format;
pub use alloc::vec;
pub use crate::err::{Error, Result};
pub use crate::mem::{memcpy, memmove, memset, memcmp, memzero};
pub use crate::arch::{inb, outb, inw, outw, io_wait};
