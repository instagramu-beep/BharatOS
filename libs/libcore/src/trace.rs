//! BharatOS libcore tracing
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::time::Duration;

bitflags::bitflags! {
    pub struct TraceFlags: u32 {
        const ENABLED       = 1 << 0;
        const CPU_BIND      = 1 << 1;
        const TIMESTAMP     = 1 << 2;
        const PROCESS       = 1 << 3;
        const THREAD        = 1 << 4;
        const ARGS          = 1 << 5;
        const RETURN_VAL    = 1 << 6;
        const SPAN          = 1 << 7;
        const LOGGING       = 1 << 8;
        const TRACE_REMOTE  = 1 << 9;
    }
}

#[repr(C)]
pub struct TraceEvent {
    pub timestamp: u128,
    pub event_type: TraceEventType,
    pub process: u32,
    pub thread: u32,
    pub args: [u64; 3],
    pub span: SpanId,
    pub len: u16,
}

#[derive(Clone, Copy)]
pub enum TraceEventType {
    FunctionEntry = 0x01,
    FunctionExit = 0x02,
    AsyncStart = 0x03,
    AsyncEnd = 0x04,
    FlowStart = 0x05,
    FlowEnd = 0x06,
    Counter = 0x07,
    ObjectNew = 0x08,
    ObjectDelete = 0x09,
    Metadata = 0x0A,
    Mark = 0x0B,
    ClockSync = 0x0C,
    ContextSwitch = 0x0D,
    CpuInfo = 0x0E,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanId(pub u64);

pub struct TraceBuffer {
    pub events: [TraceEvent; 65536],
    pub head: usize,
    pub overflow: usize,
}

impl TraceBuffer {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn record(&mut self, event: TraceEvent) {
        if self.head >= Self::CAPACITY {
            self.overflow += 1;
            return;
        }
        self.events[self.head] = event;
        self.head += 1;
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.overflow = 0;
    }

    const CAPACITY: usize = 65536;
}

static mut TRACE_BUFFER: Option<TraceBuffer> = None;

pub fn init() {
    unsafe { TRACE_BUFFER = Some(TraceBuffer::new()); }
}

pub fn record(event: TraceEvent) {
    unsafe {
        if let Some(ref mut buf) = TRACE_BUFFER {
            buf.record(event);
        }
    }
}

pub fn span_enter(name: &str) -> SpanId {
    let id = SpanId(crate::time::timestamp() as u64);
    let _ = name;
    id
}

pub fn span_exit(id: SpanId) {
    let _ = id;
}
