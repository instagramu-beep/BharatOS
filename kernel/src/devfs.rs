//! BharatOS kernel devfs
#![no_std]
#![allow(unused)]

pub use crate::fs::devfs::*;

pub struct DevFsManager {
    pub devices: [Option<DevEntry>; 256],
    pub count: usize,
}

impl DevFsManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        // Register default device nodes
        self.register_device(b"null\0", 1, 3);
        self.register_device(b"zero\0", 1, 5);
        self.register_device(b"random\0", 1, 8);
        self.register_device(b"urandom\0", 1, 9);
        self.register_device(b"mem\0", 1, 1);
        self.register_device(b"kmsg\0", 1, 11);
        self.register_device(b"tty\0", 4, 0);
        self.register_device(b"console\0", 5, 1);
        self.register_device(b"fb0\0", 29, 0);
        self.register_device(b"input\0", 13, 64);
    }

    pub fn register_device(&mut self, name: &[u8], major: u16, minor: u16) {
        let mut entry = DevEntry {
            name: [0; 32],
            major,
            minor,
            flags: DevFsFlags::CHAR,
            ops: None,
        };
        let len = name.len().min(31);
        entry.name[..len].copy_from_slice(&name[..len]);
        self.devices[self.count] = Some(entry);
        self.count += 1;
    }
}

static mut DEVFS_MANAGER: Option<DevFsManager> = None;

pub fn init() {
    unsafe { DEVFS_MANAGER = Some(DevFsManager::new()); }
    if let Some(ref mut mgr) = DEVFS_MANAGER { mgr.init(); }
}

pub fn register_device(name: &[u8], major: u16, minor: u16) {
    unsafe {
        DEVFS_MANAGER.get_or_insert(DevFsManager::new());
        DEVFS_MANAGER.as_mut().unwrap().register_device(name, major, minor);
    }
}
