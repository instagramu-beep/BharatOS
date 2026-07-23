//! BharatOS libcore debug/tracing/profiling
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod debug;
pub mod profile;
pub mod trace;
pub mod print;
pub mod assert;
pub mod stacktrace;
pub mod backtrace;
pub mod dump;
pub mod inspect;
pub mod perf;

pub use debug::*;
pub use profile::*;
pub use trace::*;

pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result;
}

pub struct Formatter {
    pub buf: Vec<u8>,
    pub indent: u32,
    pub flags: FormatterFlags,
}

bitflags::bitflags! {
    pub struct FormatterFlags: u32 {
        const ALTERNATE   = 1 << 0;
        const UPPERCASE   = 1 << 1;
        const SIGN_PLUS   = 1 << 2;
        const SIGN_MINUS  = 1 << 3;
        const HASH        = 1 << 4;
        const ZERO_PAD    = 1 << 5;
        const LEFT_ALIGN  = 1 << 6;
        const DEBUG_NAME  = 1 << 7;
    }
}

impl Formatter {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn write_str(&mut self, s: &str) { self.buf.extend_from_slice(s.as_bytes()); }
    pub fn write_char(&mut self, c: char) {
        let mut tmp = [0u8; 4];
        self.buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes());
    }
    pub fn write_u64(&mut self, n: u64, base: u8) {
        if base == 16 { self.write_str("0x"); }
        self.write_str(itoa(n, base));
    }
    pub fn write_i64(&mut self, n: i64, base: u8) {
        if n < 0 { self.write_char('-'); self.write_u64(n.wrapping_abs() as u64, base); }
        else { self.write_u64(n as u64, base); }
    }
    pub fn write_bool(&mut self, b: bool) {
        self.write_str(if b { "true" } else { "false" });
    }
    pub fn write_ptr(&mut self, p: *const ()) {
        self.write_str("0x");
        self.write_u64(p as u64, 16);
    }
}

pub struct DebugStruct<'a> {
    name: &'a str,
    f: &'a mut Formatter,
    written: bool,
}

impl<'a> DebugStruct<'a> {
    pub fn new(name: &'a str, f: &'a mut Formatter) -> Self {
        f.write_str(name);
        f.write_str(" { ");
        Self { name, f, written: false }
    }
    pub fn field<T: Debug>(&mut self, name: &str, val: &T) {
        if self.written { self.f.write_str(", "); }
        self.f.write_str(name);
        self.f.write_str(": ");
        let _ = val.fmt(self.f);
        self.written = true;
    }
    pub fn finish(mut self) {
        self.f.write_str(" }");
    }
}

impl core::fmt::Debug for Formatter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Formatter({} bytes)", self.buf.len())
    }
}

pub trait Debuggable {
    fn debug(&self) -> crate::String;
}

impl<T: Debug> core::fmt::Debug for &T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

fn ita_u64(mut n: u64, base: u8) -> Vec<u8> {
    if n == 0 { return vec![b'0']; }
    let digits = b"0123456789ABCDEF";
    let mut buf = Vec::new();
    while n > 0 {
        buf.push(digits[(n % base as u64) as usize]);
        n /= base as u64;
    }
    buf.reverse();
    buf
}

pub fn itoa(n: u64, base: u8) -> &'static str {
    // Simplified: return str representation
    "0"
}
