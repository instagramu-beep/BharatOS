//! BharatOS Clock — alarm/timer/stopwatch
#![no_std]

use crate::prelude::*;

bitflags::bitflags! {
    pub struct ClockFlags: u32 {
        const ALARM_ENABLED = 1 << 0;
        const TIMER_RUNNING = 1 << 1;
        const STOPWATCH_RUNNING = 1 << 2;
        const WORLD_CLOCK = 1 << 3;
        const SHOW_SECONDS = 1 << 4;
        const SHOW_DATE = 1 << 5;
        const SHOW_WEEKDAY = 1 << 6;
        const _24HOUR = 1 << 7;
    }
}

#[repr(C)]
pub struct ClockState {
    pub flags: ClockFlags,
    pub alarms: [Alarm; 32],
    pub alarm_count: u8,
    pub timer_remaining: u32,
    pub stopwatch_elapsed: u64,
    pub world_zones: [TimeZone; 32],
    pub timezone_count: u8,
    pub current_timezone: u8,
    pub hour_format: HourFormat,
}

#[repr(C)]
pub struct Alarm {
    pub time: u32,            // seconds since midnight
    pub label: [u8; 32],
    pub days: u8,            // bitmask 0-6 (Sun-Sat)
    pub enabled: bool,
    pub snooze_min: u8,
    pub sound: [u8; 64],
}

#[derive(Clone, Copy)]
pub struct TimeZone {
    pub name: [u8; 32],
    pub offset_minutes: i16,
    pub is_dst: bool,
    pub city: [u8; 32],
    pub country: [u8; 4],
}

#[derive(Clone, Copy, PartialEq)]
pub enum HourFormat { Hour12, Hour24 }

impl ClockState {
    pub const fn new() -> Self {
        Self {
            flags: ClockFlags::SHOW_SECONDS | ClockFlags::SHOW_DATE | ClockFlags::_24HOUR,
            alarms: unsafe { core::mem::zeroed() },
            alarm_count: 0,
            timer_remaining: 0,
            stopwatch_elapsed: 0,
            world_zones: unsafe { core::mem::zeroed() },
            timezone_count: 0,
            current_timezone: 0,
            hour_format: HourFormat::Hour24,
        }
    }

    pub fn add_alarm(&mut self, alarm: Alarm) -> u8 {
        let idx = self.alarm_count as usize;
        if idx < 32 {
            self.alarms[idx] = alarm;
            self.alarm_count += 1;
            idx as u8
        } else {
            0xFF
        }
    }

    pub fn remove_alarm(&mut self, idx: u8) {
        if (idx as usize) < self.alarm_count as usize {
            self.alarms.swap(idx as usize, self.alarm_count as usize - 1);
            self.alarm_count -= 1;
        }
    }

    pub fn check_alarms(&self) -> Option<u8> {
        let secs_of_day = (self.elapsed_secs_today()) % 86400;
        for (i, alarm) in self.alarms.iter().take(self.alarm_count as usize).enumerate() {
            if !alarm.enabled { continue; }
            let alarm_secs = alarm.time;
            if secs_of_day == alarm_secs { return Some(i as u8); }
        }
        None
    }

    fn elapsed_secs_today(&self) -> u64 {
        // Returns seconds since midnight
        let now_ts = crate::time::timestamp() as u64;
        now_ts / 1_000_000_000 % 86400
    }

    pub fn start_timer(&mut self, seconds: u32) {
        self.timer_remaining = seconds;
        self.flags.insert(ClockFlags::TIMER_RUNNING);
    }

    pub fn stop_timer(&mut self) {
        self.flags.remove(ClockFlags::TIMER_RUNNING);
    }

    pub fn tick(&mut self) {
        if self.flags.contains(ClockFlags::TIMER_RUNNING) && self.timer_remaining > 0 {
            self.timer_remaining -= 1;
        }
    }
}

impl Default for ClockState {
    fn default() -> Self { Self::new() }
}
