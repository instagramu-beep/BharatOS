//! BharatOS kernel IPC — pipes, message queues, shared memory
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const MAX_PIPES: usize = 1024;
pub const MAX_MSG_QUEUES: usize = 256;
pub const MAX_SHMEMS: usize = 512;

#[repr(C)]
pub struct Pipe {
    pub id: u32,
    pub fds: [u32; 2],
    pub buffer: [u8; 65536],
    pub head: usize,
    pub tail: usize,
    pub size: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct MsgQueue {
    pub id: u32,
    pub key: u64,
    pub max_msgs: u32,
    pub max_msg_size: u32,
    pub messages: Vec<Vec<u8>>,
}

pub struct IpcManager {
    pub pipes: [Option<Pipe>; MAX_PIPES],
    pub msg_queues: [Option<MsgQueue>; MAX_MSG_QUEUES],
    pub shmems: [Option<Shmem>; MAX_SHMEMS],
    pub pipe_count: usize,
    pub msg_queue_count: usize,
    pub shmem_count: usize,
}

#[repr(C)]
pub struct Shmem {
    pub id: u32,
    pub key: u64,
    pub addr: u64,
    pub size: usize,
    pub refcount: u32,
    pub flags: u32,
}

impl IpcManager {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(&mut self) {
        self.pipe_count = 0;
        self.msg_queue_count = 0;
        self.shmem_count = 0;
        self.pipes = unsafe { core::mem::zeroed() };
        self.msg_queues = unsafe { core::mem::zeroed() };
        self.shmems = unsafe { core::mem::zeroed() };
    }
    pub fn create_pipe(&mut self) -> Result<u32> {
        if self.pipe_count >= MAX_PIPES { return Err(crate::err::Error::Full); }
        let pipe = Pipe { id: self.pipe_count as u32, fds: [0, 0], buffer: [0; 65536], head: 0, tail: 0, size: 0, flags: 0 };
        self.pipes[self.pipe_count] = Some(pipe);
        let id = self.pipe_count as u32;
        self.pipe_count += 1;
        Ok(id)
    }
    pub fn pipe_read(&mut self, pipe_id: u32, buf: &mut [u8]) -> Result<usize> {
        let pipe = &mut self.pipes[pipe_id as usize].ok_or(crate::err::Error::NotFound)?;
        let n = pipe.size.min(buf.len());
        for i in 0..n { buf[i] = pipe.buffer[(pipe.head + i) % 65536]; }
        pipe.head = (pipe.head + n) % 65536;
        pipe.size -= n;
        Ok(n)
    }
    pub fn pipe_write(&mut self, pipe_id: u32, buf: &[u8]) -> Result<usize> {
        let pipe = &mut self.pipes[pipe_id as usize].ok_or(crate::err::Error::NotFound)?;
        if pipe.size + buf.len() > 65536 { return Err(crate::err::Error::Full); }
        for &b in buf { pipe.buffer[pipe.tail] = b; pipe.tail = (pipe.tail + 1) % 65536; }
        pipe.size += buf.len();
        Ok(buf.len())
    }
}

static mut IPC_MANAGER: Option<IpcManager> = None;

pub fn init() { unsafe { IPC_MANAGER = Some(IpcManager::new()); } if let Some(ref mut ipc) = IPC_MANAGER { ipc.init(); } }
pub fn create_pipe() -> Result<u32> { unsafe { IPC_MANAGER.as_mut().ok_or(crate::err::Error::NotSupported).and_then(|ipc| ipc.create_pipe()) } }
