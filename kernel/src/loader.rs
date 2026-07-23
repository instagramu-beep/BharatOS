//! BharatOS kernel ELF loader
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use libelf;

pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
pub const ELF_CLASS_64: u8 = 2;
pub const ELF_DATA_LSB: u8 = 1;
pub const ET_EXEC: u16 = 2;
pub const PT_LOAD: u32 = 1;

#[repr(C)]
pub struct ElfHeader64 {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader64 {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub struct LoadedElf {
    pub entry: u64,
    pub phdr_count: u16,
    pub phdr_size: u16,
    pub segments: [LoadedSegment; 16],
}

#[repr(C)]
pub struct LoadedSegment {
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub flags: u32,
}

impl LoadedElf {
    pub fn load(data: &[u8]) -> Result<Self> {
        if data.len() < 64 { return Err(crate::err::Error::InvalidMagic); }
        let header = unsafe { &*(data.as_ptr() as *const ElfHeader64) };

        if header.e_ident[0..4] != ELF_MAGIC { return Err(crate::err::Error::InvalidMagic); }
        if header.e_ident[4] != ELF_CLASS_64 { return Err(crate::err::Error::NotSupported); }

        let mut loaded = Self {
            entry: header.e_entry,
            phdr_count: header.e_phnum,
            phdr_size: header.e_phentsize,
            segments: unsafe { core::mem::zeroed() },
        };

        for i in 0..header.e_phnum.min(16) {
            let off = header.e_phoff as usize + (i as usize * header.e_phentsize as usize);
            if off + 56 > data.len() { break; }
            let phdr = unsafe { &*(data.as_ptr().add(off) as *const ProgramHeader64) };
            if phdr.p_type == PT_LOAD {
                loaded.segments[i as usize] = LoadedSegment {
                    vaddr: phdr.p_vaddr,
                    paddr: phdr.p_paddr,
                    filesz: phdr.p_filesz,
                    memsz: phdr.p_memsz,
                    flags: phdr.p_flags,
                };
            }
        }

        Ok(loaded)
    }
}
