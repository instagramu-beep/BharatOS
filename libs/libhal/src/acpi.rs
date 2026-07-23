//! BharatOS libhal ACPI — table discovery and parsing
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const ACPI_RSDP_SIG: [u8; 8] = *b"RSD PTR ";
pub const ACPI_MADT_SIG: [u8; 4] = *b"APIC";
pub const ACPI_FADT_SIG: [u8; 4] = *b"FACP";
pub const ACPI_MCFG_SIG: [u8; 4] = *b"MCFG";

#[repr(C, packed)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

#[repr(C, packed)]
pub struct Rsdp20 {
    pub rsdp: Rsdp,
    pub length: u32,
    pub xsdt_address: u64,
    pub checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: u64,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

#[repr(C, packed)]
pub struct Madt {
    pub header: SdtHeader,
    pub lapic_addr: u32,
    pub flags: u32,
}

#[repr(C, packed)]
pub struct McfgEntry {
    pub base: u64,
    pub segment: u16,
    pub bus_start: u8,
    pub bus_end: u8,
    pub reserved: u32,
}

static mut ACPI_TABLES: Option<AcpiTables> = None;

pub struct AcpiTables {
    pub rsdp: Option<&'static Rsdp>,
    pub madt: Option<&'static Madt>,
    pub mcfg: Option<&'static [McfgEntry]>,
}

impl AcpiTables {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init() -> &'static mut Self {
        unsafe {
            ACPI_TABLES = Some(Self::new());
            ACPI_TABLES.as_mut().unwrap()
        }
    }
}

pub fn acpi_init() -> usize {
    AcpiTables::init();
    0
}

pub fn find_ioapic(_id: u8) -> Option<IoApicInfo> { None }

#[repr(C)]
pub struct IoApicInfo { pub id: u8, pub address: u64, pub gsi_base: u32, pub version: u8 }
