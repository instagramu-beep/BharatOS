//! BharatOS libhal PCIe — device enumeration, config space
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
pub const PCI_CONFIG_DATA: u16 = 0xCFC;
pub const PCI_VENDOR_INTEL: u16 = 0x8086;
pub const PCI_VENDOR_AMD: u16 = 0x1022;
pub const PCI_VENDOR_NVIDIA: u16 = 0x10DE;
pub const PCI_VENDOR_BROADCOM: u16 = 0x14E4;

#[repr(C)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub header_type: u8,
    pub bar: [u32; 6],
    pub irq: u8,
    pub name: [u8; 64],
}

static mut PCI_DEVICES: [Option<PciDevice>; 256] = unsafe { core::mem::zeroed() };
static mut PCI_DEVICE_COUNT: usize = 0;

impl PciDevice {
    #[inline(always)]
    pub fn read_config(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
        let address = (1u32 << 31) | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8) | ((offset as u32) & 0xFC);
        unsafe { core::ptr::write_volatile(PCI_CONFIG_ADDRESS as *mut u32, address); core::ptr::read_volatile(PCI_CONFIG_DATA as *mut u32) }
    }

    #[inline(always)]
    pub unsafe fn write_config(&self, offset: u8, value: u32) {
        let address = (1u32 << 31) | ((self.bus as u32) << 16) | ((self.device as u32) << 11) | ((self.function as u32) << 8) | ((offset as u32) & 0xFC);
        core::ptr::write_volatile(PCI_CONFIG_ADDRESS as *mut u32, address);
        core::ptr::write_volatile(PCI_CONFIG_DATA as *mut u32, value);
    }

    pub fn bar_base(&self, idx: usize) -> u64 {
        if idx >= 6 { return 0; }
        let bar = self.bar[idx];
        if bar & 1 == 0 { (bar & 0xFFFFFFF0) as u64 } else { (bar & 0xFFFC) as u64 }
    }
}

pub fn enumerate_all() {
    unsafe {
        for bus in 0..256 {
            for device in 0..32 {
                let vendor = PciDevice::read_config(bus, device, 0, 0) as u16;
                if vendor == 0xFFFF { continue; }
                let dev_id = PciDevice::read_config(bus, device, 0, 0);
                let class_code = PciDevice::read_config(bus, device, 0, 0x0B) as u8;
                let _ = (dev_id, class_code);
                // Store device info (simplified)
            }
        }
    }
}

pub fn get_device_count() -> usize { unsafe { PCI_DEVICE_COUNT } }
pub fn get_device(_idx: usize) -> Option<&'static PciDevice> { None }
