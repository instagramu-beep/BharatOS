//! BharatOS libcore spinlock, mutex, once, RCU — minimal, no_std
#![no_std]
#![allow(unused)]

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

const UNHELD: usize = 0;
const HELD: usize = 1;

pub struct Mutex<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
    owner: AtomicUsize,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicUsize::new(UNHELD),
            data: UnsafeCell::new(data),
            owner: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        let cpu = arch_get_cpu_id();
        while self.state.compare_exchange_weak(UNHELD, HELD, Ordering::Acquire, Ordering::Relaxed).is_err() {
            unsafe { core::arch::asm!("pause") };
        }
        self.owner.store(cpu, Ordering::Release);
        MutexGuard { mutex: self }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        match self.state.compare_exchange(UNHELD, HELD, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                self.owner.store(arch_get_cpu_id(), Ordering::Release);
                Some(MutexGuard { mutex: self })
            }
            Err(_) => None,
        }
    }
}

pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a Mutex<T>,
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.owner.store(0, Ordering::Release);
        self.mutex.state.store(UNHELD, Ordering::Release);
    }
}

impl<'a, T: ?Sized> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { unsafe { &*self.mutex.data.get() } }
}

impl<'a, T: ?Sized> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.mutex.data.get() } }
}

pub struct Once {
    state: AtomicUsize,
}

const ONCE_UNINIT: usize = 0;
const ONCE_RUNNING: usize = 1;
const ONCE_DONE: usize = 2;

impl Once {
    pub const fn new() -> Self { Self { state: AtomicUsize::new(ONCE_UNINIT) } }

    pub fn call_once(&self, f: impl FnOnce()) {
        if self.state.compare_exchange(ONCE_UNINIT, ONCE_RUNNING, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            f();
            self.state.store(ONCE_DONE, Ordering::Release);
        } else {
            while self.state.load(Ordering::Acquire) != ONCE_DONE {
                unsafe { core::arch::asm!("pause") };
            }
        }
    }
}

pub struct SpinMutex<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for SpinMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicUsize::new(UNHELD),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinGuard<T> {
        while self.state.compare_exchange_weak(UNHELD, HELD, Ordering::Acquire, Ordering::Relaxed).is_err() {
            unsafe { core::arch::asm!("pause") };
        }
        SpinGuard { mutex: self }
    }
}

pub struct SpinGuard<'a, T: ?Sized> {
    mutex: &'a SpinMutex<T>,
}

impl<'a, T: ?Sized> Drop for SpinGuard<'a, T> {
    fn drop(&mut self) { self.mutex.state.store(UNHELD, Ordering::Release); }
}

impl<'a, T: ?Sized> core::ops::Deref for SpinGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { unsafe { &*self.mutex.data.get() } }
}

pub struct RCUReadGuard;

pub fn thread_yield() {
    unsafe { core::arch::asm!("pause") };
}

fn arch_get_cpu_id() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        let id: usize;
        unsafe { core::arch::asm!("mov {0}, fs:0", out(reg) id, options(nostack, preserves_flags)) };
        id
    }
    #[cfg(not(target_arch = "x86_64"))]
    0
}
