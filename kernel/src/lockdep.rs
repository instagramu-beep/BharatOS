//! BharatOS kernel lock dependency checker
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct LockDep {
    entries: [LockDepEntry; 1024],
    count: usize,
    enabled: bool,
    class_map: [LockClass; 256],
    class_count: usize,
}

#[repr(C)]
pub struct LockDepEntry {
    pub lock: *const (),
    pub class: LockClassId,
    pub acquired_at: u64,
    pub cpu: u32,
    pub depth: u16,
}

#[repr(C)]
pub struct LockClass {
    pub name: [u8; 32],
    pub key: u64,
    pub subclasses: u8,
    pub ops: u32,
}

pub type LockClassId = u16;

impl LockDep {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.enabled = cfg!(feature = "lockdep");
    }

    pub fn register_class(&mut self, name: &str, key: u64) -> LockClassId {
        let id = self.class_count as LockClassId;
        if id < 256 {
            let mut class = LockClass {
                name: [0; 32],
                key,
                subclasses: 0,
                ops: 0,
            };
            let len = name.len().min(31);
            class.name[..len].copy_from_slice(&name.as_bytes()[..len]);
            self.class_map[id as usize] = class;
            self.class_count += 1;
        }
        id
    }

    pub fn acquire(&mut self, _lock: *const (), class: LockClassId, _ip: u64) {
        if !self.enabled { return; }
        let _ = class;
    }

    pub fn release(&mut self, _lock: *const (), class: LockClassId) {
        if !self.enabled { return; }
        let _ = class;
    }

    pub fn check(&self) -> bool {
        self.enabled
    }

    pub fn report(&self) {
        let _ = self;
    }
}

static mut LOCKDEP: Option<LockDep> = None;

pub fn init() {
    unsafe {
        LOCKDEP = Some(LockDep::new());
        LOCKDEP.as_mut().unwrap().init();
    }
}

pub fn register_class(name: &str, key: u64) -> LockClassId {
    unsafe { LOCKDEP.as_mut().unwrap().register_class(name, key) }
}

pub fn acquire(lock: *const (), class: LockClassId, ip: u64) {
    unsafe { LOCKDEP.as_mut().unwrap().acquire(lock, class, ip); }
}

pub fn release(lock: *const (), class: LockClassId) {
    unsafe { LOCKDEP.as_mut().unwrap().release(lock, class); }
}
