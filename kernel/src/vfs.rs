//! BharatOS kernel VFS integration — re-exports libfs and mounts filesystems
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::fs;

pub fn init() {
    fs::vfs::init();
    // Mount root BharatFS volume
    // Mount procfs at /proc
    // Mount devfs at /dev
    // Mount tmpfs at /tmp
    kernel_log!(Init, "VFS initialized");
}

pub fn mount_root(fs: &'static dyn fs::VfsFilesystem) {
    fs::vfs::mount_root(fs);
}

pub fn mount_virtual(fs: &'static dyn fs::VfsFilesystem, path: &str) -> Result<()> {
    fs::vfs::mount_virtual(fs, path)
}

pub fn open(path: &str) -> Result<Box<dyn fs::VfsFile>> {
    let _ = path;
    Err(crate::err::Error::NotFound)
}

pub fn create(path: &str) -> Result<Box<dyn fs::VfsFile>> {
    let _ = path;
    Err(crate::err::Error::NotFound)
}

pub fn mkdir(path: &str) -> Result<()> {
    let _ = path;
    Err(crate::err::Error::NotSupported)
}

pub fn unlink(path: &str) -> Result<()> {
    let _ = path;
    Err(crate::err::Error::NotSupported)
}
