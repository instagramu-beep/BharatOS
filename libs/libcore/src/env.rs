//! BharatOS libcore environment and platform detection
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct Env {
    pub os_name: &'static str,
    pub os_version: &'static str,
    pub arch: &'static str,
    pub hostname: [u8; 64],
    pub username: [u8; 32],
    pub home_dir: [u8; 128],
    pub temp_dir: [u8; 64],
    pub path_separator: u8,
    pub max_path: u16,
    pub page_size: u32,
}

impl Env {
    pub const fn new() -> Self {
        Self {
            os_name: "BharatOS",
            os_version: "1.0.0-dev",
            arch: "x86_64",
            hostname: [0; 64],
            username: [0; 32],
            home_dir: [0; 128],
            temp_dir: [0; 64],
            path_separator: b'/',
            max_path: 4096,
            page_size: 4096,
        }
    }

    pub fn current_dir(&self) -> &str {
        "/"
    }

    pub fn set_current_dir(&mut self, _path: &str) -> Result<()> {
        Ok(())
    }

    pub fn home_dir(&self) -> &str {
        "/home/user"
    }

    pub fn temp_dir(&self) -> &str {
        "/tmp"
    }

    pub fn args() -> Vec<String> {
        Vec::new()
    }

    pub fn var(name: &str) -> Option<String> {
        let _ = name;
        None
    }

    pub fn set_var(&mut self, _name: &str, _value: &str) {
        // Set environment variable
    }

    pub fn remove_var(&mut self, _name: &str) {
        // Remove environment variable
    }

    pub fn vars(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    pub fn platform(&self) -> PlatformInfo {
        PlatformInfo {
            os: self.os_name,
            version: self.os_version,
            arch: self.arch,
            endian: Endianness::Little,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PlatformInfo {
    pub os: &'static str,
    pub version: &'static str,
    pub arch: &'static str,
    pub endian: Endianness,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Endianness { Little, Big }

static mut GLOBAL_ENV: Env = Env::new();

pub fn env() -> &'static mut Env {
    unsafe { &mut GLOBAL_ENV }
}

pub fn current_dir() -> &'static str {
    env().current_dir()
}

pub fn set_current_dir(path: &str) -> Result<()> {
    env().set_current_dir(path)
}

pub fn home_dir() -> &'static str {
    env().home_dir()
}

pub fn temp_dir() -> &'static str {
    env().temp_dir()
}
