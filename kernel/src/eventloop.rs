//! BharatOS kernel event loop for user-kernel communication
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct EventLoop {
    pub queue: Vec<KernelEvent>,
    pub max_events: usize,
    pub flags: EventLoopFlags,
}

bitflags::bitflags! {
    pub struct EventLoopFlags: u32 {
        const NONBLOCK = 1 << 0;
        const CLOSE_ON_EXEC = 1 << 1;
        const EDGE_TRIGGERED = 1 << 2;
    }
}

#[derive(Clone, Copy)]
pub struct KernelEvent {
    pub ty: KernelEventType,
    pub data: u64,
    pub userdata: u64,
    pub result: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KernelEventType {
    Readable,
    Writable,
    Error,
    HangUp,
    Invalid,
    Timer,
    Signal,
    Device,
}

impl EventLoop {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.max_events = 4096;
    }

    pub fn submit(&mut self, event: KernelEvent) -> Result<()> {
        if self.queue.len() >= self.max_events {
            return Err(crate::err::Error::Full);
        }
        self.queue.push(event);
        Ok(())
    }

    pub fn poll(&mut self) -> Option<KernelEvent> {
        self.queue.pop()
    }

    pub fn wait(&mut self) -> Option<KernelEvent> {
        if self.queue.is_empty() {
            unsafe { core::arch::asm!("hlt") };
        }
        self.queue.pop()
    }

    pub fn wake(&mut self) {
        // Wake any waiters
    }
}

static mut EVENT_LOOP: Option<EventLoop> = None;

pub fn init() {
    unsafe { EVENT_LOOP = Some(EventLoop::new()); }
    if let Some(ref mut el) = EVENT_LOOP { el.init(); }
}

pub fn submit(event: KernelEvent) -> Result<()> {
    unsafe {
        EVENT_LOOP.get_or_insert(EventLoop::new());
        EVENT_LOOP.as_mut().unwrap().submit(event)
    }
}

pub fn poll() -> Option<KernelEvent> {
    unsafe { EVENT_LOOP.as_mut().and_then(|el| el.poll()) }
}

pub fn wait() -> Option<KernelEvent> {
    unsafe { EVENT_LOOP.as_mut().and_then(|el| el.wait()) }
}
