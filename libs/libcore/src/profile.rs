//! BharatOS libcore profiling
#![no_std]
#![allow(unused)]

pub mod profile;
pub mod trace;
pub mod markers;
pub mod flamegraph;
pub mod counters;

use crate::time::{Duration, Instant};
use crate::sync::SpinMutex;

pub struct Profiler {
    pub state: ProfilerState,
    pub samples: [Sample; 16384],
    pub count: usize,
    pub current_span: Option<SpanId>,
}

#[derive(Clone, Copy)]
pub struct ProfilerState {
    pub running: bool,
    pub frequency_hz: u32,
    pub max_stack_depth: u8,
    pub cpu_target: u8,
}

#[derive(Clone, Copy)]
pub struct Sample {
    pub timestamp: u128,
    pub span_id: SpanId,
    pub cpu: u16,
    pub duration: u64,
    pub tags: [u32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanId(pub u64);

#[derive(Clone, Copy)]
pub struct Span {
    pub id: SpanId,
    pub name: [u8; 32],
    pub category: u32,
    pub start: u128,
    pub end: u128,
    pub parent: Option<SpanId>,
    pub depth: u8,
}

static mut PROFILER: Option<Profiler> = None;

impl Profiler {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn global() -> &'static mut Self {
        unsafe { PROFILER.get_or_insert(Self::new()) }
    }

    pub fn start(&mut self) {
        self.state.running = true;
    }

    pub fn stop(&mut self) {
        self.state.running = false;
    }

    pub fn begin_span(&mut self, name: &str, parent: Option<SpanId>) -> SpanId {
        let id = SpanId(Instant::now().as_nanos() as u64);
        let depth = parent.map(|_| 0).unwrap_or(0);
        self.current_span = Some(id);
        id
    }

    pub fn end_span(&mut self, id: SpanId) {
        let _ = id;
    }

    pub fn record(&mut self, span: Span) {
        let sample = Sample {
            timestamp: span.start,
            span_id: span.id,
            cpu: 0,
            duration: (span.end - span.start) as u64,
            tags: [0; 2],
        };
        if self.count < Self::CAPACITY {
            self.samples[self.count] = sample;
            self.count += 1;
        }
    }

    const CAPACITY: usize = 16384;
}

pub trait Profilable {
    fn profile(&self) -> ProfileResult;
}

pub struct ProfileResult {
    pub total_duration_ns: u64,
    pub self_duration_ns: u64,
    pub call_count: u32,
    pub min_duration_ns: u64,
    pub max_duration_ns: u64,
    pub avg_duration_ns: u64,
    pub children: Vec<ProfileResult>,
}

impl ProfileResult {
    pub fn total_time(&self) -> Duration {
        Duration::from_nanos(self.total_duration_ns)
    }
    pub fn self_time(&self) -> Duration {
        Duration::from_nanos(self.self_duration_ns)
    }
}
