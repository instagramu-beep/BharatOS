//! BharatOS libsched — CFS + RT hybrid scheduler with per-CPU run queues
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::mm;

pub const MAX_TASKS: usize = 1024;
pub const RT_PRIORITY_BAND: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(pub usize);

impl core::ops::Deref for TaskId { type Target = usize; fn deref(&self) -> &Self::Target { &self.0 } }

#[derive(Clone, Copy, Debug)]
pub struct Task {
    pub pid: usize,
    pub tid: usize,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub vruntime: u64,
    pub stack_base: u64,
    pub kernel_stack: u64,
    pub entry: fn(),
    pub name: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskState { New, Ready, Running, Blocked, Zombie, Exited, Suspended, Parked }

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

    pub fn from_nice(nice: i8) -> Self {
        let effective = (50 - nice as i16).clamp(0, 99) as u8;
        Self { nice, rt_prio: 0, effective }
    }
}

pub struct FairRunQueue {
    pub tasks: [Option<Task>; 256],
    pub count: usize,
    pub min_vruntime: u64,
}

impl FairRunQueue {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn push(&mut self, task: Task) {
        if self.count < 256 { self.tasks[self.count] = Some(task); self.count += 1; }
    }
    pub fn pick(&mut self) -> Option<Task> {
        if self.count == 0 { return None; }
        let idx = (self.min_vruntime % self.count as u64) as usize;
        let task = self.tasks[idx].take()?;
        self.tasks.swap(idx, self.count - 1);
        self.count -= 1;
        Some(task)
    }
}

pub struct RtRunQueue {
    pub queue: [Option<TaskId>; 64],
    pub head: usize,
    pub tail: usize,
}

impl RtRunQueue {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
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

pub struct PerCpuRunQueue {
    pub fair: FairRunQueue,
    pub rt: RtRunQueue,
    pub current: Option<TaskId>,
}

pub struct Scheduler {
    pub per_cpu: [PerCpuRunQueue; 4],
    pub task_table: [Option<Task>; MAX_TASKS],
    pub next_pid: usize,
    pub ticks: u64,
    pub quantum_ns: u64,
}

impl Scheduler {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    pub fn init(&mut self, quantum_ns: u64) {
        self.quantum_ns = quantum_ns;
        self.next_pid = 2; // skip idle
        self.ticks = 0;
        for i in 0..4 {
            self.per_cpu[i] = PerCpuRunQueue { fair: FairRunQueue::new(), rt: RtRunQueue::new(), current: None };
        }
        // Spawn idle threads
        for cpu in 0..4 {
            let idle = Task {
                pid: 1,
                tid: 1,
                state: TaskState::Running,
                priority: TaskPriority::IDLE,
                vruntime: 0,
                stack_base: 0,
                kernel_stack: 0,
                entry: idle_thread,
                name: *b"idle\0",
            };
            self.task_table[cpu] = Some(idle);
            self.per_cpu[cpu].current = Some(TaskId(cpu));
        }
    }

    pub fn spawn_kthread(&mut self, entry: fn(), name: &str, prio: u8) -> Result<TaskId> {
        let pid = self.next_pid;
        self.next_pid += 1;
        let task = Task {
            pid,
            tid: pid,
            state: TaskState::Ready,
            priority: TaskPriority { nice: 0 - prio as i8, rt_prio: 0, effective: prio },
            vruntime: 0,
            stack_base: 0,
            kernel_stack: 0,
            entry,
            name: name_bytes(name),
        };
        self.task_table[pid] = Some(task);
        self.per_cpu[0].fair.push(self.task_table[pid].unwrap());
        Ok(TaskId(pid))
    }

    pub fn spawn_user(&mut self, entry: u64) -> Result<TaskId> {
        let pid = self.next_pid;
        self.next_pid += 1;
        let task = Task {
            pid,
            tid: pid,
            state: TaskState::Ready,
            priority: TaskPriority::NORMAL,
            vruntime: 0,
            stack_base: entry,
            kernel_stack: 0,
            entry: spawn_user_entry,
            name: *b"user\0",
        };
        self.task_table[pid] = Some(task);
        self.per_cpu[0].fair.push(self.task_table[pid].unwrap());
        Ok(TaskId(pid))
    }

    pub fn on_timer_tick(&mut self, cpu: usize) -> Option<TaskId> {
        self.ticks += 1;
        self.schedule(cpu)
    }

    pub fn schedule(&mut self, cpu: usize) -> Option<TaskId> {
        if let Some(rt) = self.per_cpu[cpu].rt.pick() { return Some(rt); }
        if let Some(fair) = self.per_cpu[cpu].fair.pick() { return Some(TaskId(fair.pid)); }
        Some(TaskId(cpu)) // fallback to idle
    }

    pub fn yield_current(&mut self, cpu: usize) {
        let _ = self.schedule(cpu);
    }

    pub fn block_current(&mut self, cpu: usize) {
        if let Some(current) = self.per_cpu[cpu].current {
            if let Some(task) = self.task_table[current.0].as_mut() {
                task.state = TaskState::Blocked;
            }
        }
        self.per_cpu[cpu].current = None;
    }

    pub fn wake(&mut self, tid: TaskId) {
        if let Some(task) = self.task_table[tid.0].as_mut() {
            task.state = TaskState::Ready;
            self.per_cpu[0].fair.push(*task);
        }
    }
}

static mut SCHEDULER: Option<Scheduler> = None;

fn name_bytes(s: &str) -> [u8; 32] {
    let mut buf = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(31);
    buf[..len].copy_from_slice(&bytes[..len]);
    buf
}

fn idle_thread() -> ! { loop { unsafe { core::arch::asm!("hlt") }; } }

fn spawn_user_entry() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

pub fn init(quantum_ns: u64) {
    unsafe {
        SCHEDULER = Some(Scheduler::new());
        SCHEDULER.as_mut().unwrap().init(quantum_ns);
    }
}

pub fn sched() -> Option<TaskId> {
    unsafe { SCHEDULER.as_mut()?.schedule(0) }
}

pub fn tick() -> Option<TaskId> {
    unsafe { SCHEDULER.as_mut()?.on_timer_tick(0) }
}

pub fn spawn_kthread(entry: fn(), name: &str, prio: u8) -> Result<TaskId> {
    unsafe { SCHEDULER.as_mut().ok_or(crate::err::Error::NotSupported)?.spawn_kthread(entry, name, prio) }
}

pub fn yield_current() {
    unsafe { if let Some(ref mut s) = SCHEDULER { s.yield_current(0); } }
}

pub fn block_current() {
    unsafe { if let Some(ref mut s) = SCHEDULER { s.block_current(0); } }
}

pub fn wake(tid: TaskId) {
    unsafe { if let Some(ref mut s) = SCHEDULER { s.wake(tid); } }
}

pub fn get_scheduler() -> Option<&'static mut Scheduler> {
    unsafe { SCHEDULER.as_mut() }
}
