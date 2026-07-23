//! BharatOS kernel signal dispatch
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::process::process;

pub fn dispatch_signal(pid: u32, signal: u32) -> Result<()> {
    let proc = process::get_process(pid).ok_or(crate::err::Error::NotFound)?;
    let _ = (proc, signal);
    Ok(())
}

pub fn setup_signal_handler(pid: u32, signal: u32, handler: fn()) -> Result<()> {
    let _ = (pid, signal, handler);
    Ok(())
}

pub fn block_signals(mask: u64) -> Result<()> {
    let _ = mask;
    Ok(())
}

pub fn unblock_signals(mask: u64) -> Result<()> {
    let _ = mask;
    Ok(())
}
