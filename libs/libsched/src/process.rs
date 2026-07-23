//! BharatOS libsched process management
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::{mm, fs};

pub const MAX_PROCESSES: usize = 4096;
pub const MAX_THREADS_PER_PROCESS: usize = 256;
pub const MAX_FDS: usize = 1024;
pub const MAX_ARGV: usize = 128;
pub const MAX_ENVP: usize = 128;

pub type Pid = u32;
pub type Tid = u32;
pub type Fd = u32;

#[repr(C)]
pub struct Process {
    pub pid: Pid,
    pub ppid: Pid,
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub state: TaskState,
    pub ty: TaskType,
    pub vm_base: u64,
    pub vm_size: u64,
    pub kernel_stack: u64,
    pub page_dir: u64,
    pub flags: TaskFlags,
    pub limits: TaskLimits,
    pub stats: TaskStats,
    pub priority: TaskPriority,
    pub cmd_line: [u8; 256],
    pub cwd: [u8; 256],
    pub threads: [Option<Tid>; MAX_THREADS_PER_PROCESS],
    pub thread_count: u16,
    pub fds: [Option<FdDesc>; MAX_FDS],
    pub fd_count: u16,
    pub next: Option<Pid>,
}

#[repr(C)]
pub struct FdDesc {
    pub fd: Fd,
    pub flags: u32,
    pub pos: u64,
    pub file: Option<&'static fs::VfsNode>,
    pub private: *mut (),
}

#[repr(C)]
pub struct Thread {
    pub tid: Tid,
    pub pid: Pid,
    pub state: TaskState,
    pub stack_base: u64,
    pub stack_size: u64,
    pub instruction_ptr: u64,
    pub stack_ptr: u64,
    pub registers: [u64; 16],
    pub fpu_state: [u8; 512],
    pub priority: TaskPriority,
    pub scheduler_data: [u64; 8],
}

static mut PROCESS_TABLE: [Option<&'static mut Process>; MAX_PROCESSES] = unsafe { core::mem::zeroed() };
static mut NEXT_PID: Pid = 1;

pub fn init() {
    // Create init process (PID 1)
    let init = create_process(0);
    unsafe {
        PROCESS_TABLE[init.pid as usize] = Some(init);
    }
}

pub fn create_process(parent_pid: Pid) -> &'static mut Process {
    let pid = alloc_pid();
    unsafe {
        // Allocate from static table in real implementation
        let proc = Process {
            pid,
            ppid: parent_pid,
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            state: TaskState::New,
            ty: TaskType::UserProcess,
            vm_base: 0x1000000,
            vm_size: 0,
            kernel_stack: 0,
            page_dir: 0,
            flags: TaskFlags::empty(),
            limits: TaskLimits::default(),
            stats: TaskStats::default(),
            priority: TaskPriority::NORMAL,
            cmd_line: [0; 256],
            cwd: [0; 256],
            threads: [None; MAX_THREADS_PER_PROCESS],
            thread_count: 0,
            fds: [None; MAX_FDS],
            fd_count: 0,
            next: None,
        };
        let boxed: Box<Process> = Box::new(proc);
        Box::leak(boxed)
    }
}

pub fn create_thread(proc: &Process) -> &'static mut Thread {
    let tid = alloc_pid();
    Thread::new(tid, proc.pid)
}

pub fn fork(parent: &Process) -> Result<&'static mut Process> {
    let child = create_process(parent.pid);
    // Copy memory, fds, etc.
    Ok(child)
}

pub fn exec(proc: &mut Process, path: &str, argv: &[&str], envp: &[&str]) -> Result<()> {
    // Load ELF and replace address space
    Ok(())
}

pub fn exit(proc: &mut Process, code: i32) {
    proc.state = TaskState::Zombie;
    proc.stats.exit_code = code;
}

pub fn wait(pid: Pid) -> Option<&'static mut Process> {
    unsafe { PROCESS_TABLE[pid as usize].take() }
}

pub fn get_process(pid: Pid) -> Option<&'static Process> {
    unsafe { PROCESS_TABLE[pid as usize as usize].as_ref().map(|p| p as &Process) }
}

pub fn current_process() -> Option<&'static mut Process> {
    let cpu = 0usize;
    let current = libsched::schedule(cpu);
    current.and_then(|tid| get_process(tid.0 as Pid).map(|p| p as &mut Process))
}

fn alloc_pid() -> Pid {
    unsafe {
        let pid = NEXT_PID;
        NEXT_PID += 1;
        pid
    }
}
