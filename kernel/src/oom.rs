//! BharatOS kernel OOM (Out-of-Memory) killer
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::mm;

pub enum OomAction {
    None,
    Warn,
    KillOne,
    KillSeveral,
    Panic,
}

pub struct OomState {
    pub pressure: u8,
    pub last_kill: u64,
    pub killed_count: u64,
    pub pressure_frac: u8,
    pub badness_fn: fn(u32) -> u64,
}

impl OomState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn evaluate(&mut self) -> OomAction {
        let mm = mm::get_mm();
        let avail = mm.map(|m| m.free_pages).unwrap_or(0);
        let total = mm.map(|m| m.total_pages).unwrap_or(1);

        self.pressure_frac = (100 - (avail * 100 / total)) as u8;

        if self.pressure_frac > 95 {
            OomAction::KillSeveral
        } else if self.pressure_frac > 80 {
            OomAction::KillOne
        } else if self.pressure_frac > 60 {
            OomAction::Warn
        } else {
            OomAction::None
        }
    }

    pub fn select_victim(&self) -> Option<u32> {
        let _ = self;
        // Select worst offender by RSS
        None
    }

    pub fn kill(&mut self, pid: u32) {
        self.last_kill = crate::time::timestamp() as u64;
        self.killed_count += 1;
        let _ = pid;
    }
}

static mut OOM_STATE: Option<OomState> = None;

pub fn init() {
    unsafe { OOM_STATE = Some(OomState::new()); }
}

pub fn check_oom() {
    unsafe {
        if let Some(ref mut oom) = OOM_STATE {
            match oom.evaluate() {
                OomAction::KillOne => {
                    if let Some(pid) = oom.select_victim() {
                        oom.kill(pid);
                    }
                }
                OomAction::KillSeveral => {
                    for _ in 0..3 {
                        if let Some(pid) = oom.select_victim() {
                            oom.kill(pid);
                        }
                    }
                }
                OomAction::Panic => {
                    kernel_panic!("OOM: system out of memory");
                }
                _ => {}
            }
        }
    }
}

pub fn pressure() -> u8 {
    unsafe { OOM_STATE.as_ref().map(|o| o.pressure_frac).unwrap_or(0) }
}
