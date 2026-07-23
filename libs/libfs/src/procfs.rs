//! BharatOS procfs — process filesystem
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::sched::process;

pub struct ProcFs;

impl ProcFs {
    pub fn new() -> Self { Self {} }

    pub fn proc_entries(&self) -> Vec<&'static str> {
        vec!["cpuinfo", "meminfo", "uptime", "version", "mounts", "self", "cmdline", "environ"]
    }
}

static PROC_ROOT: SpinMutex<Option<VfsNode>> = SpinMutex::new(None);

impl VfsFilesystem for ProcFs {
    fn root(&self) -> &'static VfsNode {
        unsafe {
            let mut guard = PROC_ROOT.lock();
            if guard.is_none() {
                *guard = Some(VfsNode {
                    ino: 1,
                    ty: VfsNodeType::Directory,
                    flags: VfsFlags::RDONLY | VfsFlags::NOEXEC,
                    size: 0,
                    ops: &PROC_OPS,
                    private: core::ptr::null_mut(),
                    parent: None,
                    children: SpinMutex::new(Vec::new()),
                });
            }
            &*(guard.as_ref().unwrap() as *const VfsNode)
        }
    }
    fn name(&self) -> &str { "proc" }
}

static PROC_OPS: VfsOps = VfsOps {
    open: |_, _| Err(err::Error::NotSupported),
    create: |_, _| Err(err::Error::NotSupported),
    mkdir: |_, _| Err(err::Error::NotSupported),
    readdir: |_| Ok(Vec::new()),
    lookup: |_, _| Err(err::Error::NotFound),
    ioctl: |_, _, _| Ok(Vec::new()),
};

pub fn show_cpuinfo() -> String {
    let info = libhal::cpu::detect();
    format!(
        "processor\t: 0\nvendor_id\t: {:?}\ncore_count\t: {}\nmodel name\t: {}\n",
        info.vendor, info.core_count, info.brand_string(),
    )
}

pub fn show_meminfo() -> Vec<u8> {
    b"MemTotal: 8192000 kB\nMemFree: 4096000 kB\nBuffers: 102400 kB\nCached: 2048000 kB\n".to_vec()
}

pub fn show_uptime() -> Vec<u8> {
    let up = crate::libhal::timer::uptime_secs();
    let idle = 0u64;
    format!("{} {}\n", up, idle).into_bytes()
}

pub fn show_version() -> Vec<u8> {
    format!("BharatOS {}.{}.{}-dev\n", crate::VERSION_MAJOR, crate::VERSION_MINOR, crate::VERSION_PATCH).into_bytes()
}
