//! BharatOS libhal PMU (Performance Monitoring Unit)
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct PmuFlags: u32 { const ENABLED = 1 << 0; const OVERFLOW = 1 << 1; }
}

#[derive(Clone, Copy)]
pub enum PmuEvent { CpuCycles, Instructions, CacheMisses, BranchMisses }

pub struct PmuCounter { pub index: u32, pub event: PmuEvent, pub count: u64, pub flags: PmuFlags }
pub struct Pmu { pub counters: [Option<PmuCounter>; 8], pub count: usize }

impl Pmu {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(&mut self) { unsafe { libhal::msr::write(0x38F, 0xF); } }
    pub fn allocate(&mut self, event: PmuEvent, _flags: PmuFlags) -> Option<&mut PmuCounter> {
        if self.count >= 8 { return None; }
        let c = PmuCounter { index: self.count as u32, event, count: 0, flags: PmuFlags::ENABLED };
        self.counters[self.count] = Some(c);
        self.count += 1;
        self.counters.get_mut(self.count - 1)?.as_mut().into()
    }
    pub fn read_all(&mut self) {
        for c in &mut self.counters[..self.count] {
            if let Some(ref mut counter) = c {
                counter.count = crate::time::rdtsc();
            }
        }
    }
}
