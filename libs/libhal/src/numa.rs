//! BharatOS libhal NUMA topology
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const MAX_NUMA_NODES: usize = 64;

#[repr(C)]
pub struct NumaNode { pub id: u8, pub base: u64, pub length: u64, pub cpu_mask: u64, pub zone_count: u8 }

static mut NUMA_NODES: [Option<NumaNode>; MAX_NUMA_NODES] = unsafe { core::mem::zeroed() };
static mut NUMA_COUNT: usize = 0;

pub fn detect_and_build_topology() {
    unsafe {
        NUMA_NODES[0] = Some(NumaNode { id: 0, base: 0, length: 0x100000000, cpu_mask: 1, zone_count: 1 });
        NUMA_COUNT = 1;
    }
}
pub fn get_node_count() -> usize { unsafe { NUMA_COUNT } }
pub fn get_node(_id: usize) -> Option<&'static mut NumaNode> {
    unsafe { NUMA_NODES.get_mut(_id).and_then(|n| n.as_mut()) }
}
pub fn node_for_addr(addr: u64) -> Option<u8> {
    unsafe { NUMA_NODES.iter_mut().find_map(|n| n.as_ref().and_then(|nn| if addr >= nn.base && addr < nn.base + nn.length { Some(nn.id) } else { None })) }
}
