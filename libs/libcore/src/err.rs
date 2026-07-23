//! BharatOS libcore error types, recover strategy, and no-std融媒体 Result
#![no_std]
use crate::prelude::*;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ErrorClass: u32 {
        const IO        = 1 << 0;
        const MEM       = 1 << 1;
        const SEC       = 1 << 2;
        const FS        = 1 << 3;
        const NET       = 1 << 4;
        const AUI       = 1 << 5;
        const KERNEL    = 1 << 6;
        const DRIVER    = 1 << 7;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ErrorFlags: u32 {
        const RETRYABLE   = 1 << 0;
        const FATAL       = 1 << 1;
        const RECOVERABLE = 1 << 2;
        const TRIGGER_CORE_DUMP = 1 << 3;
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Error {
    pub class: ErrorClass,
    pub code: u16,
    pub flags: ErrorFlags,
    pub context: [u8; 16],
}

#[macro_export]
macro_rules! err {
    ($class:expr, $code:expr) => { $crate::err::Error { class: $class, code: $code, flags: $crate::err::ErrorFlags::empty(), context: [0; 16] } };
    ($class:expr, $code:expr, $flag:expr) => { $crate::err::Error { class: $class, code: $code, flags: $flag, context: [0; 16] } };
}

impl Error {
    pub const IO: Self = err!(ErrorClass::IO, 1);
    pub const NOT_FOUND: Self = err!(ErrorClass::IO, 2);
    pub const NO_MEM: Self = err!(ErrorClass::MEM, 1);
    pub const SECTOR_FAULT: Self = err!(ErrorClass::SEC, 1);
    pub const FS_CORRUPT: Self = err!(ErrorClass::FS, 1);
    pub const NET_DISCONNECTED: Self = err!(ErrorClass::NET, 1);
    pub const DRIVER_NOT_INITIALIZED: Self = err!(ErrorClass::DRIVER, 1);
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{:?}/{:04x}] {:064b}", self.class, self.code, self.flags)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn recover_strategy(err: &Error) -> RecoverStrategy {
    if err.flags.contains(ErrorFlags::RETRYABLE) { RecoverStrategy::Retry }
    else if err.flags.contains(ErrorFlags::RECOVERABLE) { RecoverStrategy::Fallback }
    else if err.flags.contains(ErrorFlags::FATAL) { RecoverStrategy::CoreDump {
        signal: Signal::SIGSEGV, reason: err.code,
    }
    }
    else { RecoverStrategy::Ignore }
}

#[derive(Clone, Copy, Debug)]
pub enum RecoverStrategy {
    Retry,
    Fallback,
    Ignore,
    CoreDump { signal: Signal, reason: u16 },
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Signal {
    SIGKILL   = 9,
    SIGSEGV   = 11,
    SIGILL    = 4,
    SIGBUS    = 7,
    SIGFPE    = 8,
    SIGTERM   = 15,
}
