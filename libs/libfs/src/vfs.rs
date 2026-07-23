//! BharatOS VFS — virtual filesystem layer
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod vfs;
pub mod bhfs;
pub mod procfs;
pub mod devfs;
pub mod tmpfs;

pub use vfs::*;

bitflags::bitflags! {
    pub struct VfsFlags: u32 {
        const RDONLY = 1 << 0;
        const NOEXEC = 1 << 1;
        const NODEV  = 1 << 2;
        const SYNCS  = 1 << 3;
    }
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
    pub children: SpinMutex<Vec<&'static VfsNode>>,
}

#[repr(C)]
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
pub struct DirEnt {
    pub ino: u64,
    pub ty: VfsNodeType,
    pub name: [u8; 255],
}

static ROOT: SpinMutex<Option<&'static VfsNode>> = SpinMutex::new(None);

pub fn init() {
    unsafe {
        *ROOT.lock() = Some(BHARATFS_MOUNT);
    }
    let _ = mount_devfs("/dev");
    let _ = mount_procfs("/proc");
    let _ = mount_tmpfs("/tmp");
}

pub fn mount_root(fs: &'static dyn VfsFilesystem) {
    let root = fs.root();
    unsafe {
        *ROOT.lock() = Some(root);
    }
}

pub fn mount_devfs(path: &str) -> Result<()> {
    let fs = crate::devfs::DevFs::new();
    let _ = path;
    let _ = fs;
    Ok(())
}

pub fn mount_procfs(path: &str) -> Result<()> {
    let fs = crate::procfs::ProcFs::new();
    let _ = path;
    let _ = fs;
    Ok(())
}

pub fn mount_tmpfs(path: &str) -> Result<()> {
    let mut fs = crate::tmpfs::TmpFsManager::new();
    fs.init();
    fs.mkdir(path).ok();
    Ok(())
}

pub trait VfsFilesystem {
    fn root(&self) -> &'static VfsNode;
    fn name(&self) -> &str;
}
