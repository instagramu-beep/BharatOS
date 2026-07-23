//! BharatOS libhal IOMMU — device DMA address translation
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IommuKind { AmdVi, IntelDmar, ArmSmmu, RiscvSbi }

bitflags::bitflags! {
    pub struct IommuFlags: u64 { const TRANSLATION = 1 << 0; const INTERRUPT_REMAP = 1 << 1; }
}

pub struct IommuDevice { pub id: u64, pub kind: IommuKind, pub base: u64, pub flags: IommuFlags }
pub struct IommuDomain { pub id: u32, pub page_table: u64 }

impl IommuDevice {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(&mut self) -> core::result::Result<()> { self.flags.insert(IommuFlags::TRANSLATION); Ok(()) }
    pub fn map_page(&self, _d: &IommuDomain, _iova: u64, _phys: u64) -> core::result::Result<()> { Ok(()) }
    pub fn unmap_page(&self, _d: &IommuDomain, _iova: u64) -> core::result::Result<()> { Ok(()) }
}
