//! BharatOS libcore IPC — sockets, pipes, shared memory, message queues
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod socket;
pub mod pipe;
pub mod shm;
pub mod msgqueue;
pub mod unix_socket;
pub mod netlink;

pub use socket::*;
pub use pipe::*;
pub use shm::*;
pub use msgqueue::*;

bitflags::bitflags! {
    pub struct SocketFlags: u32 {
        const CLOEXEC   = 1 << 0;
        const NONBLOCK  = 1 << 1;
        const PASS_CRED = 1 << 2;
        const NOSIGPIPE = 1 << 3;
    }
}

#[repr(C)]
pub struct MsgHdr {
    pub name: *const u8,
    pub namelen: u32,
    pub iov: *const IoVec,
    pub iovlen: usize,
    pub control: *mut u8,
    pub control_len: usize,
    pub flags: MsgFlags,
}

bitflags::bitflags! {
    pub struct MsgFlags: i32 {
        const TRUNC    = 1 << 0;
        const CTRUNC   = 1 << 1;
        const OOB      = 1 << 2;
        const DONTWAIT = 1 << 3;
        const MORE     = 1 << 4;
        const NOSIGNAL = 1 << 5;
        const WAITALL  = 1 << 6;
    }
}

#[repr(C)]
pub struct IoVec {
    pub base: *mut u8,
    pub len: usize,
}

pub struct Mmap {
    pub addr: u64,
    pub len: usize,
    pub prot: u32,
    pub flags: u32,
    pub fd: u32,
    pub offset: u64,
}

bitflags::bitflags! {
    pub struct MmapProt: u32 {
        const READ   = 1 << 0;
        const WRITE  = 1 << 1;
        const EXEC   = 1 << 2;
    }
    pub struct MmapFlags: u32 {
        const SHARED  = 1 << 0;
        const PRIVATE = 1 << 1;
        const FIXED   = 1 << 2;
        const ANONYMOUS = 1 << 3;
    }
}

pub struct Pipe {
    pub read_fd: Fd,
    pub write_fd: Fd,
    pub buffer: [u8; 65536],
    pub head: usize,
    pub tail: usize,
    pub size: usize,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            read_fd: 0,
            write_fd: 0,
            buffer: [0; 65536],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let available = self.size;
        let n = available.min(buf.len());
        for i in 0..n {
            buf[i] = self.buffer[(self.head + i) % 65536];
        }
        self.head = (self.head + n) % 65536;
        self.size -= n;
        Ok(n)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.size + buf.len() > 65536 { return Err(crate::err::Error::WouldBlock); }
        for &byte in buf {
            self.buffer[self.tail] = byte;
            self.tail = (self.tail + 1) % 65536;
        }
        self.size += buf.len();
        Ok(buf.len())
    }

    pub fn available(&self) -> usize { 65536 - self.size }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    pub fn is_full(&self) -> bool { self.size == 65536 }
}

pub struct ShmRegion {
    pub key: u64,
    pub addr: u64,
    pub size: usize,
    pub refcount: u32,
    pub flags: ShmFlags,
}

bitflags::bitflags! {
    pub struct ShmFlags: u32 {
        const READ  = 1 << 0;
        const WRITE = 1 << 1;
        const EXEC  = 1 << 2;
        const LOCKED = 1 << 3;
        const HUGE  = 1 << 4;
    }
}

pub struct MsgQueue {
    pub key: u64,
    pub max_msgs: u32,
    pub max_msg_size: u32,
    pub messages: Vec<Vec<u8>>,
    pub send_blocked: bool,
}

impl MsgQueue {
    pub fn send(&mut self, msg: &[u8]) -> Result<()> {
        if self.messages.len() >= self.max_msgs as usize {
            return Err(crate::err::Error::WouldBlock);
        }
        self.messages.push(msg.to_vec());
        Ok(())
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(msg) = self.messages.pop() {
            let n = msg.len().min(buf.len());
            buf[..n].copy_from_slice(&msg[..n]);
            Ok(n)
        } else {
            Err(crate::err::Error::WouldBlock)
        }
    }
}
