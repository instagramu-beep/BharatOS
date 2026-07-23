//! BharatOS libhal DMA (Direct Memory Access)
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum DmaDirection { MemToMem, MemToDev, DevToMem, DevToDev }

pub struct DmaChannel { pub id: u8, pub device: u16, pub direction: DmaDirection }

pub struct DmaController { pub channels: [Option<DmaChannel>; 8], pub base: u16 }

impl DmaController {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(&mut self) {
        unsafe {
            for ch in 0..8 {
                let _ = ch;
            }
        }
    }
    pub fn allocate_channel(&mut self, _device: u16) -> Option<DmaChannel> { None }
}

pub fn init_dma() {
    DmaController::new();
}
