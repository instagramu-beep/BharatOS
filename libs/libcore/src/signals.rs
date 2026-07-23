//! BharatOS libcore signals — POSIX-style signal delivery
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct SignalFlags: u64 {
        const SIGKILL   = 1 << 9;
        const SIGTERM   = 1 << 15;
        const SIGSTOP   = 1 << 17;
        const SIGCONT   = 1 << 18;
        const SIGINT    = 1 << 2;
        const SIGQUIT   = 1 << 3;
        const SIGILL    = 1 << 4;
        const SIGTRAP   = 1 << 5;
        const SIGABRT   = 1 << 6;
        const SIGBUS    = 1 << 7;
        const SIGFPE    = 1 << 8;
        const SIGSEGV   = 1 << 11;
        const SIGPIPE   = 1 << 13;
        const SIGALRM   = 1 << 14;
        const SIGUSR1   = 1 << 10;
        const SIGUSR2   = 1 << 12;
        const SIGCHLD   = 1 << 20;
        const SIGPWR    = 1 << 29;
        const SIGSTKFLT = 1 << 16;
        const SIGIO     = 1 << 23;
        const SIGTTOU   = 1 << 22;
        const SIGTTIN   = 1 << 21;
        const SIGTSTP   = 1 << 19;
        const SIGWINCH  = 1 << 28;
        const SIGURG    = 1 << 23;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Signal {
    SIGHUP  = 1,
    SIGINT  = 2,
    SIGQUIT = 3,
    SIGILL  = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS  = 7,
    SIGFPE  = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGSTKFLT = 16,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG  = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO   = 29,
    SIGPWR  = 30,
    SIGSYS  = 31,
}

impl Signal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Signal::SIGHUP => "SIGHUP",
            Signal::SIGINT => "SIGINT",
            Signal::SIGQUIT => "SIGQUIT",
            Signal::SIGILL => "SIGILL",
            Signal::SIGSEGV => "SIGSEGV",
            Signal::SIGKILL => "SIGKILL",
            Signal::SIGTERM => "SIGTERM",
            Signal::SIGSTOP => "SIGSTOP",
            Signal::SIGCONT => "SIGCONT",
            Signal::SIGFPE => "SIGFPE",
            Signal::SIGBUS => "SIGBUS",
            Signal::SIGALRM => "SIGALRM",
            Signal::SIGUSR1 => "SIGUSR1",
            Signal::SIGUSR2 => "SIGUSR2",
            Signal::SIGCHLD => "SIGCHLD",
            Signal::SIGSYS => "SIGSYS",
            Signal::SIGPIPE => "SIGPIPE",
            _ => "UNKNOWN",
        }
    }

    pub fn is_default_fatal(&self) -> bool {
        matches!(self, Signal::SIGILL | Signal::SIGSEGV | Signal::SIGBUS |
            Signal::SIGFPE | Signal::SIGSYS | Signal::SIGTRAP)
    }

    pub fn is_default_stop(&self) -> bool {
        matches!(self, Signal::SIGSTOP | Signal::SIGTSTP | Signal::SIGTTIN |
            Signal::SIGTTOU)
    }

    pub fn is_default_continue(&self) -> bool {
        matches!(self, Signal::SIGCONT)
    }

    pub fn is_default_ignore(&self) -> bool {
        matches!(self, Signal::SIGCHLD)
    }

    pub fn is_core_dump(&self) -> bool {
        matches!(self, Signal::SIGQUIT | Signal::SIGILL | Signal::SIGABRT |
            Signal::SIGFPE | Signal::SIGSEGV | Signal::SIGBUS | Signal::SIGSYS)
    }
}

bitflags::bitflags! {
    pub struct SigActionFlags: u32 {
        const RESTART    = 1 << 0;
        const NODEFER    = 1 << 1;
        const RESETHAND  = 1 << 2;
        const ONSTACK    = 1 << 3;
        const SIGINFO    = 1 << 4;
        const SA_RESTORER = 1 << 5;
    }
}

#[repr(C)]
pub struct SigAction {
    pub handler: fn(Signal),
    pub flags: SigActionFlags,
    pub mask: SignalFlags,
    pub restorer: fn(),
}

static mut SIG_HANDLERS: [Option<SigAction>; 32] = unsafe { core::mem::zeroed() };

pub fn init() {
    // Set default handlers
}

pub fn set_handler(sig: Signal, action: SigAction) -> Result<()> {
    unsafe {
        SIG_HANDLERS[sig as usize] = Some(action);
    }
    Ok(())
}

pub fn send(pid: Pid, sig: Signal) -> Result<()> {
    let _ = pid;
    let _ = sig;
    Ok(())
}

pub fn raise(sig: Signal) -> Result<()> {
    send(process::current_pid(), sig)
}

pub fn block(mask: SignalFlags) -> Result<()> {
    Ok(())
}

pub fn unblock(mask: SignalFlags) -> Result<()> {
    Ok(())
}

pub fn pending() -> SignalFlags {
    SignalFlags::empty()
}
