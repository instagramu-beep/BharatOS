//! BharatOS libsched task states and lifecycle
#![no_std]
#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskState {
    New = 0,
    Ready = 1,
    Running = 2,
    Blocked = 3,
    Zombie = 4,
    Exited = 5,
    Suspended = 6,
    Parked = 7,
    Dying = 8,
}

#[derive(Clone, Copy, Debug)]
pub enum TaskType {
    UserProcess,
    KernelThread,
    IdleThread,
    RealTime,
    Background,
}

#[derive(Clone, Copy, Debug)]
pub struct TaskPriority {
    pub nice: i8,
    pub rt_prio: u8,
    pub effective: u8,
}

impl TaskPriority {
    pub const IDLE: Self = Self { nice: 19, rt_prio: 0, effective: 0 };
    pub const LOW: Self = Self { nice: 10, rt_prio: 0, effective: 20 };
    pub const NORMAL: Self = Self { nice: 0, rt_prio: 0, effective: 50 };
    pub const HIGH: Self = Self { nice: -10, rt_prio: 0, effective: 80 };
    pub const RT_MIN: Self = Self { nice: 0, rt_prio: 1, effective: 90 };
    pub const RT_MAX: Self = Self { nice: 0, rt_prio: 99, effective: 99 };

    pub fn from_nice(nice: i8) -> Self {
        let effective = (50 - nice as i16).clamp(0, 99) as u8;
        Self { nice, rt_prio: 0, effective }
    }

    pub fn effective(&self) -> u8 {
        if self.rt_prio > 0 { return self.rt_prio; }
        self.effective
    }
}

bitflags::bitflags! {
    pub struct TaskFlags: u64 {
        const EXITING        = 1 << 0;
        const NEED_SCHED     = 1 << 1;
        const FPU_OWNED      = 1 << 2;
        const NO_PREEMPT     = 1 << 3;
        const KERNEL_STACK   = 1 << 4;
        const WAIT_INTERRUPTIBLE = 1 << 5;
        const WAKE_ON_FORK   = 1 << 6;
        const OOM_DISABLE    = 1 << 7;
        const NO_NEW_PRIVS   = 1 << 8;
        const START_DEADD    = 1 << 9;
    }
}

#[derive(Clone, Copy)]
pub struct TaskLimits {
    pub cpu_time_us: u64,
    pub cpu_soft_limit_us: u64,
    pub cpu_hard_limit_us: u64,
    pub memory_bytes: u64,
    pub mem_soft_limit: u64,
    pub mem_hard_limit: u64,
    pub file_count: u32,
    pub thread_count: u32,
    pub nice: i8,
    pub rt_priority: u8,
    pub io_priority: u8,
    pub cpus_allowed: u64,
    pub numa_node: u8,
}

impl Default for TaskLimits {
    fn default() -> Self {
        Self {
            cpu_time_us: 0,
            cpu_soft_limit_us: u64::MAX,
            cpu_hard_limit_us: u64::MAX,
            memory_bytes: 0,
            mem_soft_limit: u64::MAX,
            mem_hard_limit: u64::MAX,
            file_count: 1024,
            thread_count: 1024,
            nice: 0,
            rt_priority: 0,
            io_priority: 4,
            cpus_allowed: u64::MAX,
            numa_node: 0xFF,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TaskStats {
    pub voluntary_ctx_switches: u64,
    pub involuntary_ctx_switches: u64,
    pub user_time_ns: u64,
    pub system_time_ns: u64,
    pub minor_page_faults: u64,
    pub major_page_faults: u64,
    pub io_bytes_read: u64,
    pub io_bytes_written: u64,
    pub children_user_time_ns: u64,
    pub children_system_time_ns: u64,
}

impl Default for TaskStats {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
