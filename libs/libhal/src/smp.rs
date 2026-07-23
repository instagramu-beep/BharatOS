//! BharatOS libhal SMP bring-up
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::hal::apic;

pub const MAX_CPUS: usize = 256;

static mut CPU_LOCAL_DATA: [Option<CpuLocalData>; MAX_CPUS] = unsafe { core::mem::zeroed() };
static mut CPU_COUNT: usize = 0;

#[repr(C)]
pub struct CpuLocalData { pub cpu_id: u32, pub lapic_id: u32, pub kernel_stack: u64, pub gs_base: u64, pub irq_count: u64 }

pub fn smp_bootstrap() {
    unsafe {
        CPU_LOCAL_DATA[0] = Some(CpuLocalData { cpu_id: 0, lapic_id: 0, kernel_stack: 0, gs_base: 0, irq_count: 0 });
        CPU_COUNT = 1;
    }
}

pub fn wake_ap(_cpu_id: u32, _entry: u64) {
    unsafe {
        let lapic = apic::read_apic_base() as *mut u32;
        apic::write_register(lapic, apic::LAPIC_ICR_LO, 0x00004500 | (_cpu_id << 24));
    }
}

pub fn ap_entry() {
    unsafe {
        let id = CPU_COUNT;
        CPU_LOCAL_DATA[id] = Some(CpuLocalData { cpu_id: id as u32, lapic_id: 0, kernel_stack: 0, gs_base: 0, irq_count: 0 });
        CPU_COUNT += 1;
    }
    loop { unsafe { core::arch::asm!("hlt") }; }
}

pub fn cpu_count() -> usize { unsafe { CPU_COUNT } }
pub fn this_cpu() -> usize { 0 }
pub fn is_bsp() -> bool { true }
