//! BharatOS kernel module declarations — finalized
#![no_std]
#![allow(unused)]

pub mod arch;
pub mod syscalls;
pub mod boot;
pub mod process;
pub mod loader;
pub mod memory;
pub mod mm;
pub mod frame;
pub mod heap;
pub mod paging;
pub mod slab;
pub mod oom;
pub mod vfs;
pub mod bhfs;
pub mod procfs;
pub mod devfs;
pub mod tmpfs;
pub mod network;
pub mod ipc;
pub mod security;
pub mod signals;
pub mod signals_dispatch;
pub mod lock;
pub mod lockdep;
pub mod utils;
pub mod debug;
pub mod tracing;
pub mod profiling;
pub mod unified_logger;
pub mod eventbus;
pub mod eventloop;
pub mod architecture;

pub use crate::process::TaskId;
pub use crate::error::{Error, Result};
pub use crate::arch::x86_64::gdt::Gdt;

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};
