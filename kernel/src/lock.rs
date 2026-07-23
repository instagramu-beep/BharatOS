//! BharatOS kernel lock primitives
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct SpinLock {
    state: AtomicU8,
    owner: AtomicU32,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(0),
            owner: AtomicU32::new(0),
        }
    }

    pub fn lock(&self) -> LockGuard {
        while self.state.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            unsafe { core::arch::asm!("pause") };
        }
        self.owner.store(current_cpu(), Ordering::Release);
        LockGuard { lock: self }
    }
}

pub struct LockGuard<'a> {
    lock: &'a SpinLock,
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
        self.lock.owner.store(0, Ordering::Release);
    }
}

pub struct RwLock {
    readers: AtomicU32,
    writer: AtomicU8,
}

impl RwLock {
    pub const fn new() -> Self {
        Self {
            readers: AtomicU32::new(0),
            writer: AtomicU8::new(0),
        }
    }

    pub fn read(&self) -> ReadGuard {
        loop {
            if self.writer.load(Ordering::Acquire) == 0 {
                self.readers.fetch_add(1, Ordering::Acquire);
                if self.writer.load(Ordering::Acquire) == 0 { break; }
                self.readers.fetch_sub(1, Ordering::Release);
            }
        }
        ReadGuard { lock: self }
    }

    pub fn write(&self) -> WriteGuard {
        while self.writer.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            unsafe { core::arch::asm!("pause") };
        }
        while self.readers.load(Ordering::Acquire) > 0 {
            unsafe { core::arch::asm!("pause") };
        }
        WriteGuard { lock: self }
    }
}

pub struct ReadGuard<'a> { lock: &'a RwLock }
pub struct WriteGuard<'a> { lock: &'a RwLock }

impl<'a> Drop for ReadGuard<'a> {
    fn drop(&mut self) { self.lock.readers.fetch_sub(1, Ordering::Release); }
}
impl<'a> Drop for WriteGuard<'a> {
    fn drop(&mut self) { self.lock.writer.store(0, Ordering::Release); }
}

pub struct Seqlock {
    sequence: AtomicU32,
}

impl Seqlock {
    pub const fn new() -> Self { Self { sequence: AtomicU32::new(0) } }

    pub fn read_start(&self) -> u32 { self.sequence.load(Ordering::Acquire) }
    pub fn read_retry(&self, start: u32) -> bool { self.sequence.load(Ordering::Acquire) != start || self.sequence.load(Ordering::Relaxed) & 1 != 0 }
    pub fn write_lock(&self) { loop { let _ = self.sequence.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed); if self.sequence.load(Ordering::Relaxed) == 0 { break; } } }
    pub fn write_unlock(&self) { self.sequence.store(0, Ordering::Release); }
}

pub fn current_cpu() -> u32 { 0 }
