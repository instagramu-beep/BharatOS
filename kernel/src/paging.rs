//! BharatOS kernel paging — 4-level page tables for x86_64
#![no_std]
#![allow(unused)]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct PtFlags: u64 {
        const PRESENT   = 1 << 0;
        const WRITABLE  = 1 << 1;
        const USER      = 1 << 2;
        const HUGE      = 1 << 3;
        const GLOBAL    = 1 << 4;
        const NO_EXEC   = 1 << 5;
        const COW       = 1 << 6;
        const DEVICE    = 1 << 8;
        const SWAPPED   = 1 << 9;
    }
}

const PML4_INDEX_SHIFT: u64 = 39;
const PDP_INDEX_SHIFT: u64 = 30;
const PD_INDEX_SHIFT: u64 = 21;
const PT_INDEX_SHIFT: u64 = 12;

#[repr(C, align(4096))]
pub struct PageTable([u64; 512]);

impl PageTable {
    pub const fn new() -> Self {
        Self([0; 512])
    }

    pub fn set_entry(&mut self, idx: usize, phys: u64, flags: PtFlags) {
        self.0[idx] = (phys & 0x000F_FFFF_FFFF_F000) | flags.bits();
    }

    pub fn get_entry(&self, idx: usize) -> Option<u64> {
        let entry = self.0[idx];
        if (entry & PtFlags::PRESENT.bits()) != 0 { Some(entry) } else { None }
    }

    pub fn map_page(&mut self, virt: u64, phys: u64, flags: PtFlags) {
        let pml4_idx = ((virt >> PML4_INDEX_SHIFT) & 0x1FF) as usize;
        let pdp_idx = ((virt >> PDP_INDEX_SHIFT) & 0x1FF) as usize;
        let pd_idx = ((virt >> PD_INDEX_SHIFT) & 0x1FF) as usize;
        let pt_idx = ((virt >> PT_INDEX_SHIFT) & 0x1FF) as usize;

        let pml4 = &mut self.0;
        if pml4[pml4_idx] & PtFlags::PRESENT.bits() == 0 {
            let new = PageTable::new();
            pml4[pml4_idx] = new.0[0] | PtFlags::PRESENT.bits() | PtFlags::WRITABLE.bits();
        }

        let pdp = (pml4[pml4_idx] & !0xFFF) as *mut u64;
        unsafe {
            if (*pdp.add(pdp_idx) & PtFlags::PRESENT.bits()) == 0 {
                let new = PageTable::new();
                *pdp.add(pdp_idx) = new.0[0] | PtFlags::PRESENT.bits() | PtFlags::WRITABLE.bits();
            }
            let pd = (*pdp.add(pdp_idx) & !0xFFF) as *mut u64;
            if (*pd.add(pd_idx) & PtFlags::PRESENT.bits()) == 0 {
                let new = PageTable::new();
                *pd.add(pd_idx) = new.0[0] | PtFlags::PRESENT.bits() | PtFlags::WRITABLE.bits();
            }
            let pt = (*pd.add(pd_idx) & !0xFFF) as *mut u64;
            *pt.add(pt_idx) = phys | flags.bits();
        }
    }
}

pub fn init_identity_map() {
    // Map first 4 GB identity-mapped and higher-half kernel
}
