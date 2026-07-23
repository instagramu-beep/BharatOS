//! BharatOS devfs — virtual devices filesystem
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct DevFsFlags: u32 {
        const CHAR      = 1 << 0;
        const BLOCK     = 1 << 1;
        const FIFO      = 1 << 2;
        const SOCKET    = 1 << 3;
    }
}

#[derive(Clone, Copy)]
pub struct DevEntry {
    pub name: [u8; 32],
    pub major: u16,
    pub minor: u16,
    pub flags: DevFsFlags,
    pub ops: Option<DevOps>,
}

pub struct DevOps {
    pub open: fn(&DevEntry) -> Result<Box<dyn Device>>,
    pub read: fn(&dyn Device, &mut [u8]) -> Result<usize>,
    pub write: fn(&mut dyn Device, &[u8]) -> Result<usize>,
    pub ioctl: fn(&dyn Device, u32, &[u8]) -> Result<Vec<u8>>,
}

pub trait Device {
    fn read(&self, buf: &mut [u8]) -> Result<usize> { Ok(0) }
    fn write(&mut self, buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
    fn ioctl(&self, cmd: u32, arg: &[u8]) -> Result<Vec<u8>> { Ok(Vec::new()) }
    fn poll(&self, _mask: u8) -> u8 { 0 }
    fn close(&mut self) -> Result<()> { Ok(()) }
}

pub struct DevFsRoot {
    pub entries: Vec<DevEntry>,
    pub kernel_log: KernelLogDev,
    pub null: NullDev,
    pub zero: ZeroDev,
    pub random: RandomDev,
    pub urandom: RandomDev,
    pub mem: MemDev,
    pub port: PortDev,
    pub console: ConsoleDev,
}

impl DevFsRoot {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.register_device(b"null\0", 1, 3, DevFsFlags::CHAR);
        self.register_device(b"zero\0", 1, 5, DevFsFlags::CHAR);
        self.register_device(b"random\0", 1, 8, DevFsFlags::CHAR);
        self.register_device(b"urandom\0", 1, 9, DevFsFlags::CHAR);
        self.register_device(b"mem\0", 1, 1, DevFsFlags::CHAR);
        self.register_device(b"kmsg\0", 1, 11, DevFsFlags::CHAR);
        self.register_device(b"tty\0", 4, 0, DevFsFlags::CHAR);
        self.register_device(b"console\0", 5, 1, DevFsFlags::CHAR);
    }

    fn register_device(&mut self, name: &[u8], major: u16, minor: u16, flags: DevFsFlags) {
        let mut entry = DevEntry {
            name: [0; 32], major, minor, flags,
            ops: None,
        };
        entry.name[..name.len()].copy_from_slice(name);
        self.entries.push(entry);
    }
}

pub struct KernelLogDev;
pub struct NullDev;
pub struct ZeroDev;
pub struct RandomDev;
pub struct MemDev;
pub struct PortDev;
pub struct ConsoleDev;

impl Device for NullDev {
    fn read(&self, _buf: &mut [u8]) -> Result<usize> { Ok(0) }
    fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(_buf.len()) }
}

impl Device for ZeroDev {
    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        for b in buf.iter_mut() { *b = 0; }
        Ok(buf.len())
    }
}
