//! BharatOS libelf — ELF binary parser and loader
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod parser;
pub mod linker;
pub mod relocations;
pub mod symbols;

bitflags::bitflags! {
    pub struct ElfFlags: u32 {
        const EXECUTABLE = 1 << 0;
        const RELOCATABLE = 1 << 1;
        const DYNAMIC = 1 << 2;
        const STATIC = 1 << 3;
        const PIE = 1 << 4;
        const DEBUG = 1 << 5;
    }
}

#[derive(Clone, Copy)]
pub enum ElfClass { Elf32, Elf64 }

#[derive(Clone, Copy, PartialEq)]
pub enum ElfType { None, Rel, Exec, Dyn, Core }
