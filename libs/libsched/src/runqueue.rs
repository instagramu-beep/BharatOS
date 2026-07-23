//! BharatOS libsched runqueue implementations
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::process::Task;

pub struct FairRunQueue {
    pub tasks: [Option<&'static mut Task>; 256],
    pub count: usize,
    pub min_vruntime: u64,
}

impl FairRunQueue {
    pub const fn new() -> Self {
        Self {
            tasks: unsafe { core::mem::zeroed() },
            count: 0,
            min_vruntime: 0,
        }
    }

    pub fn push(&mut self, task: &'static mut Task) {
        if self.count < 256 {
            self.tasks[self.count] = Some(task);
            self.count += 1;
        }
    }

    pub fn pick(&mut self) -> Option<TaskId> {
        if self.count == 0 { return None; }
        let idx = self.min_vruntime % self.count as u64;
        let task = self.tasks[idx as usize].take()?;
        self.tasks.swap(idx as usize, self.count - 1);
        self.count -= 1;
        Some(TaskId(task.pid))
    }
}

pub struct RtRunQueue {
    pub queue: [Option<TaskId>; 64],
    pub head: usize,
    pub tail: usize,
}

impl RtRunQueue {
    pub const fn new() -> Self {
        Self { queue: unsafe { core::mem::zeroed() }, head: 0, tail: 0 }
    }

    pub fn push(&mut self, id: TaskId) {
        if (self.tail + 1) % 64 != self.head {
            self.queue[self.tail] = Some(id);
            self.tail = (self.tail + 1) % 64;
        }
    }

    pub fn pick(&mut self) -> Option<TaskId> {
        if self.head == self.tail { return None; }
        let id = self.queue[self.head]?;
        self.head = (self.head + 1) % 64;
        Some(id)
    }
}
