//! BharatOS Mont — Live memory instrumentation daemon
//!
//! Collects per-process, global, kernel-heap and page-cache snapshots from the
//! kernel and exposes them as BPF/BPF trace / netlink events.

#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::net::ipc::{netlink, netevent};

pub mod sampler;
pub mod vmacache;
pub mod profile;
pub mod leak;

pub struct MontState {
    pub global_mem: u64,           // bytes available
    pub used: u64,                 // bytes occupied
    pub slab: u64,
    pub percpu_cache: u64,
    pub hugepages: u32,
    pub oom_pressure: u8,
    pub top_5_consumers: [ProcessMem; 5],
    pub leak_candidates: [u32; 16],
}

pub struct ProcessMem {
    pub pid: u32,
    pub label: [u8; 32],
    pub rss: u64,
    pub virt: u64,
    pub shared: u64,
    pub oom_adj: i8,
}

pub struct MontDaemon {
    pub state: MontState,
    pub interval_ms: u32,
    pub leak_threshold: u64,
    pub panics: Vec<u64>,
}

const fn default() -> Mont {
    MontDaemon {
        state: MontState {
            global_mem: 0, used: 0, slab: 0, percpu_cache: 0,
            hugepages: 0, oom_pressure: 0,
            top_5_consumers: [ProcessMem::default(); 5],
            leak_candidates: [0; 16],
        },
        interval_ms: 1000,
        leak_threshold: 4096, // 4 MB
        panics: Vec::new(),
    }
}

impl MontDaemon {
    pub fn run(&mut self) -> ! {
        loop {
            self.poll();
            core::hint::spin_loop();
        }
    }
    
    fn poll(&mut self) {
        self.poll_kernel_snap();
        self.detect_leaks();
        self.launder();
    }
    
    fn poll_kernel_snap(&mut self) {
        let snap = sampler::collect_snapshot();
        self.state = snap;
        self.notify_consumer_map();
    }
    
    fn detect_leaks(&mut self) {
        let prev = self.state.used;
        self.poll_kernel_snap();
        let diff = self.state.used.wrapping_sub(prev);
        if diff > self.leak_threshold {
            leak::capture(&mut self.state.leak_trace);
            self.warn_memory_pressure(diff);
        }
    }

    fn launder(&mut self) {
        // Drop unused objects in the kernel kheap, madvise anonymous regions
        self.drop_idle(10000); // older than 10 s
        self.compact_page_cache();
    }
    
    fn notify_consumer_map(&self) {
        let ev = netevent::RawEvent {
            type_: netlink::events::MEM_STAT,
            pid: 0,
            payload: unsafe {
                core::slice::from_raw_parts(&self.state as *const _ as *const u8,
                                           size_of_val(&self.state))
            },
        };
        netlink::broadcast(&ev);
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct SampledStats {
    pub total_phys: u64,
    pub free: u64,
    pub slab_bytes: u64,
    pub file_map: u64,
    pub anon: u64,
    pub available: u64,
    pub reclaimable: u64,
    pub active: u64,
    pub inactive: u64,
    pub zone_counts: [u64; 10],
}

impl Default for SampledStats { fn default() -> Self { unsafe { core::mem::zeroed() } } }
