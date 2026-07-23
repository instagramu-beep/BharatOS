//! BharatOS kernel module declarations — cross-crate kernel types
#![no_std]
#![allow(unused)]

pub mod types;
pub mod process;
pub mod thread;
pub mod memory;
pub mod mm;
pub mod namespace;
pub mod object;
pub mod ucore;
pub mod uhandler;

pub use types::*;
pub use process::*;
pub use thread::*;
pub use memory::*;
pub use mm::*;

#[repr(C, packed)]
pub struct Process {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub state: ProcessState,
    pub vm_base: u64,
    pub vm_size: u64,
    pub mem_map: u64, // physical base of page tables
    pub flags: u64,
    pub cmd: [u8; 64],
    pub cwd: [u8; 256],
    pub thread_count: u16,
    pub _reserved: [u8; 34],
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Blocked,
    Zombie,
    Exited,
    Suspended,
}

#[repr(C)]
pub struct ResourceLimits {
    pub cpu_soft: u64,
    pub cpu_hard: u64,
    pub mem_soft: u64,
    pub mem_hard: u64,
    pub fd_limit: u32,
    pub task_limit: u32,
    pub nice: i32,
    pub rt_priority: u32,
    pub virtual_address_space: u64,
    pub shared_segment_quota: u64,
}
