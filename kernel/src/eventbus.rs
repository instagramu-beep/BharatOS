//! BharatOS kernel event bus — publish-subscribe for inter-subsystem communication
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const MAX_EVENTS: usize = 4096;

bitflags::bitflags! {
    pub struct EventBusFlags: u32 {
        const DROP_OLDEST = 1 << 0;
        const DROP_NEWEST = 1 << 1;
        const BROADCAST = 1 << 2;
        const PERSISTENT = 1 << 3;
    }
}

#[derive(Clone, Copy)]
pub struct Event {
    pub ty: EventType,
    pub source: u32,
    pub data: [u64; 4],
    pub timestamp: u128,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventType {
    ProcessEvent,
    IoEvent,
    NetEvent,
    PowerEvent,
    SecurityEvent,
    DeviceEvent,
    UserEvent,
    SystemEvent,
    TimerEvent,
    CustomEvent(u32),
}

pub struct EventBus {
    pub events: [Option<Event>; MAX_EVENTS],
    pub head: usize,
    pub tail: usize,
    pub count: usize,
    pub subscribers: [Option<EventCallback>; 64],
    pub subscriber_count: usize,
}

pub type EventCallback = fn(Event);

impl EventBus {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
        self.subscriber_count = 0;
        self.events = unsafe { core::mem::zeroed() };
        self.subscribers = unsafe { core::mem::zeroed() };
    }

    pub fn publish(&mut self, event: Event) {
        if self.count >= MAX_EVENTS {
            return;
        }
        self.events[self.tail] = Some(event);
        self.tail = (self.tail + 1) % MAX_EVENTS;
        self.count += 1;
        self.dispatch();
    }

    pub fn subscribe(&mut self, cb: EventCallback) -> u32 {
        if self.subscriber_count < 64 {
            self.subscribers[self.subscriber_count] = Some(cb);
            self.subscriber_count += 1;
            (self.subscriber_count - 1) as u32
        } else {
            0xFFFFFFFF
        }
    }

    pub fn dispatch(&mut self) {
        let count = self.count;
        for _ in 0..count.min(16) {
            let event = self.events[self.head].take().unwrap();
            for i in 0..self.subscriber_count {
                if let Some(cb) = self.subscribers[i] {
                    cb(event);
                }
            }
        }
    }

    pub fn poll(&mut self) -> Option<Event> {
        if self.count == 0 { return None; }
        let event = self.events[self.head].take().unwrap();
        self.head = (self.head + 1) % MAX_EVENTS;
        self.count -= 1;
        Some(event)
    }
}

static mut EVENT_BUS: Option<EventBus> = None;

pub fn init() {
    unsafe { EVENT_BUS = Some(EventBus::new()); }
}

pub fn publish(event: Event) {
    unsafe {
        if let Some(ref mut bus) = EVENT_BUS {
            bus.publish(event);
        }
    }
}

pub fn subscribe(cb: EventCallback) -> u32 {
    unsafe {
        EVENT_BUS.get_or_insert(EventBus::new());
        EVENT_BUS.as_mut().unwrap().subscribe(cb)
    }
}
