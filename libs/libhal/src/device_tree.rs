//! BharatOS libhal device tree (FDT) for ARM/RISC-V
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const FDT_MAGIC: u32 = 0xD00DFEED;

#[repr(C)]
pub struct FdtHeader {
    pub magic: u32,
    pub total_size: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub off_dt_strings_size: u32,
    pub off_dt_struct_size: u32,
}

#[derive(Clone, Copy)]
pub struct DeviceTreeNode {
    pub name: &'static str,
    pub compatible: &'static str,
    pub children: Vec<DeviceTreeNode>,
}

pub struct DeviceTree { pub header: &'static FdtHeader, pub root_node: DeviceTreeNode }

impl DeviceTree {
    pub fn parse(_data: &[u8]) -> Result<Self> {
        let _ = _data;
        Ok(DeviceTree { header: unsafe { &*(0 as *const FdtHeader) }, root_node: DeviceTreeNode { name: "/", compatible: "", children: Vec::new() } })
    }
}
