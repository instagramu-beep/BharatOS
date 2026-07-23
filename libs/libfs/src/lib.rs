//! BharatOS libfs — virtual filesystem layer + BharatFS driver
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct VfsFlags: u32 { const RDONLY = 1 << 0; const NOEXEC = 1 << 1; const NODEV = 1 << 2; const SYNCS = 1 << 3; }
}

#[derive(Clone, Copy, Debug)]
pub enum VfsNodeType { File, Directory, Symlink, CharDev, BlockDev, Socket, Pipe }

#[repr(C)]
pub struct VfsNode {
    pub ino: u64,
    pub ty: VfsNodeType,
    pub flags: VfsFlags,
    pub size: u64,
    pub ops: &'static VfsOps,
    pub private: *mut (),
    pub parent: Option<&'static VfsNode>,
}

pub struct VfsOps {
    pub open: fn(&VfsNode, &str) -> Result<Box<dyn VfsFile>>,
    pub create: fn(&VfsNode, &str) -> Result<Box<dyn VfsFile>>,
    pub mkdir: fn(&VfsNode, &str) -> Result<&'static VfsNode>,
    pub readdir: fn(&VfsNode) -> Result<Vec<DirEnt>>,
    pub lookup: fn(&VfsNode, &str) -> Result<&'static VfsNode>,
    pub ioctl: fn(&VfsNode, u32, &[u8]) -> Result<Vec<u8>>,
}

pub trait VfsFile: Sync {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn seek(&mut self, off: u64) -> Result<u64>;
    fn flush(&mut self) -> Result<()>;
    fn close(self: Box<Self>) -> Result<()>;
}

#[repr(C)]
pub struct DirEnt { pub ino: u64, pub ty: VfsNodeType, pub name: [u8; 255] }

pub trait VfsFilesystem {
    fn root(&self) -> &'static VfsNode;
    fn name(&self) -> &str;
}

static mut VFS_ROOT: Option<&'static VfsNode> = None;

pub fn init() {
    unsafe { VFS_ROOT = None; }
}

pub fn mount_root(_fs: &'static dyn VfsFilesystem) {
    unsafe { VFS_ROOT = Some(_fs.root()); }
}

pub fn mount_virtual(_fs: &'static dyn VfsFilesystem, _path: &str) -> Result<()> { Ok(()) }

pub mod bhfs;
pub mod procfs;
pub mod devfs;
pub mod tmpfs;
pub mod path;

pub use bhfs::*;
pub use procfs::*;
pub use devfs::*;
