//! BharatOS libhal per-CPU local storage
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct CpuLocal<T: 'static> {
    data: [Option<&'static mut T>; 256],
}

impl<T> CpuLocal<T> {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn get(&mut self, cpu: usize) -> Option<&mut T> { self.data.get_mut(cpu).and_then(|d| d.as_deref_mut()) }
    pub fn set(&mut self, cpu: usize, data: &'static mut T) { self.data[cpu] = Some(data); }
    pub fn current(&mut self) -> Option<&mut T> { self.get(current_cpu()) }
}

pub fn current_cpu() -> usize { 0 }
