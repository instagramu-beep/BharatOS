//! BharatOS libcore event system
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct EventFlags: u32 {
        const READABLE    = 1 << 0;
        const WRITABLE    = 1 << 1;
        const ERROR       = 1 << 2;
        const TIMEOUT     = 1 << 3;
        const SIGNAL      = 1 << 4;
        const EDGE        = 1 << 5;
        const LEVEL       = 1 << 6;
        const ONE_SHOT    = 1 << 7;
    }
}

pub struct EventLoop;
pub struct EventListener;
pub struct EventSource;
pub struct EventPoller;

impl EventLoop {
    pub fn new() -> Self { Self {} }
    pub fn poll(&self) -> Option<Event> { None }
    pub fn run(&self) -> ! { loop {} }
    pub fn wakeup(&self) {
        let _ = self;
    }
}

impl EventListener {
    pub fn new() -> Self { Self {} }
}

#[derive(Clone, Copy)]
pub struct Event {
    pub ty: EventType,
    pub data: u64,
    pub userdata: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EventType { Read, Write, Error, Signal, Timeout, Custom }
