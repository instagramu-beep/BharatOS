//! BharatOS kernel procfs
#![no_std]
#![allow(unused)]

pub use crate::fs::procfs::*;

pub fn proc_init() {
    // Initialize procfs with initial entries
}

pub fn proc_register_process(pid: u32) {
    // Register a new process in procfs
}

pub fn proc_unregister_process(pid: u32) {
    // Unregister a process from procfs
}

pub fn proc_update_stats(pid: u32) {
    // Update /proc/<pid>/stat and /proc/<pid>/status
}
