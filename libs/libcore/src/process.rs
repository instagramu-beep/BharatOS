//! BharatOS libcore process abstractions
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct ProcessFlags: u32 {
        const EXITING        = 1 << 0;
        const NEED_SCHED     = 1 << 1;
        const FPU_OWNED      = 1 << 2;
        const NO_PREEMPT     = 1 << 3;
        const OOM_DISABLE    = 1 << 7;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Blocked,
    Zombie,
    Exited,
}

#[repr(C)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub state: ProcessState,
    pub flags: ProcessFlags,
    pub cmd_line: [u8; 256],
    pub utime: u64,
    pub stime: u64,
    pub memory: u64,
    pub threads: u16,
}

impl ProcessInfo {
    pub const fn new(pid: u32) -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub fn current_pid() -> u32 { 1 }
pub fn current_tid() -> u32 { 1 }
pub fn current_uid() -> u32 { 0 }
pub fn current_gid() -> u32 { 0 }
pub fn get_process_info(pid: u32) -> Option<ProcessInfo> { None }
pub fn list_processes() -> Vec<ProcessInfo> { Vec::new() }
pub fn fork() -> Result<u32> {
    let pid = unsafe { libsched::Scheduler::with(|s| s.next_pid) };
    Err(err::Error::NotSupported)
}

pub fn exec(path: &str) -> Result<()> {
    use crate::fs::vfs;
    let _ = path;
    match vfs::lookup("/bin/init") {
        Some(_) => Ok(()),
        None => Err(err::Error::NotFound),
    }
}
pub fn exit(code: i32) -> ! {
    loop {
        unsafe { libsched::yield_current(); }
        unsafe { core::arch::asm!("hlt"); }
    }
}
pub fn wait(pid: u32) -> Option<i32> { None }
pub fn kill(pid: u32, sig: u32) -> Result<()> { Ok(()) }
