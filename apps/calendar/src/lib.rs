//! BharatOS Calendar App
#![no_std]
#![allow(unused)]

use libcore::prelude::*;

bitflags::bitflags! {
    pub struct CalendarFlags: u32 {
        const SHOW_WEEK_NUMBERS = 1 << 0;
        const SHOW_EVENTS = 1 << 1;
        const SHOW_HOLIDAYS = 1 << 2;
        const WEEK_START_MONDAY = 1 << 3;
        const MULTI_VIEW = 1 << 4;
        const REMINDERS = 1 << 5;
        const SYNC = 1 << 6;
        const LUNAR = 1 << 7;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode { Month, Week, Day, Year, Agenda, Schedule }

#[derive(Clone, Copy, PartialEq)]
pub enum EventKind { Appointment, Meeting, Reminder, Holiday, Birthday, Task, AllDay, MultiDay }

#[repr(C)]
pub struct CalendarEvent {
    pub id: u64,
    pub title: [u8; 128],
    pub description: [u8; 512],
    pub location: [u8; 128],
    pub start_ms: u64,
    pub end_ms: u64,
    pub all_day: bool,
    pub kind: EventKind,
    pub color: u32,
    pub reminders: [Reminder; 8],
    pub reminder_count: u8,
    pub recurrence: RecurrenceRule,
    pub attendees: [Attendee; 32],
    pub attendee_count: u8,
    pub calendar_id: u32,
    pub flags: EventFlags,
}

bitflags::bitflags! {
    pub struct EventFlags: u32 {
        const ALL_DAY = 1 << 0;
        const RECURRING = 1 << 1;
        const PRIVATE = 1 << 2;
        const TENTATIVE = 1 << 3;
        const CANCELLED = 1 << 4;
        const BUSY = 1 << 5;
        const AVAILABLE = 1 << 6;
        const REMINDER_SET = 1 << 7;
    }
}

#[repr(C)]
pub struct Reminder {
    pub minutes_before: u16,
    pub method: ReminderMethod,
    pub triggered: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ReminderMethod { Notification, Email, SMS, Popup }

#[derive(Clone, Copy)]
pub struct RecurrenceRule {
    pub frequency: RecurrenceFreq,
    pub interval: u16,
    pub by_day: [u8; 7],
    pub by_month_day: [u8; 31],
    pub count: u32,
    pub until_ms: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RecurrenceFreq { None, Daily, Weekly, BiWeekly, Monthly, Yearly, Custom }

#[repr(C)]
pub struct Attendee {
    pub email: [u8; 128],
    pub name: [u8; 64],
    pub status: AttendeeStatus,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AttendeeStatus { Unknown, Accepted, Declined, Tentative, Pending }

#[repr(C)]
pub struct CalendarAccount {
    pub id: u32,
    pub name: [u8; 64],
    pub kind: CalendarKind,
    pub color: u32,
    pub visible: bool,
    pub sync_enabled: bool,
    pub owner: [u8; 128],
}

#[derive(Clone, Copy, PartialEq)]
pub enum CalendarKind { Local, CalDav, Google, Outlook, ICloud, Exchange }

pub struct CalendarState {
    pub flags: CalendarFlags,
    pub view: ViewMode,
    pub current_date_ms: u64,
    pub selected_date_ms: u64,
    pub events: Vec<CalendarEvent>,
    pub calendars: [CalendarAccount; 16],
    pub calendar_count: u8,
    pub selected_calendar_mask: u64,
}

impl CalendarState {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.flags.insert(CalendarFlags::SHOW_EVENTS);
        self.view = ViewMode::Month;
        self.current_date_ms = crate::time::timestamp() as u64;
        self.selected_date_ms = self.current_date_ms;
        self.calendar_count = 1;
        self.calendars[0] = CalendarAccount {
            id: 0,
            name: *b"My Calendar\0",
            kind: CalendarKind::Local,
            color: 0xFF4285F4,
            visible: true,
            sync_enabled: false,
            owner: *b"",
        };
    }

    pub fn add_event(&mut self, event: CalendarEvent) -> u64 {
        let id = (self.events.len() as u64) + 1;
        self.events.push(event);
        id
    }

    pub fn remove_event(&mut self, id: u64) {
        self.events.retain(|e| e.id != id);
    }

    pub fn get_events_for_day(&self, day_ms: u64) -> Vec<&CalendarEvent> {
        let day_start = day_ms / 86400000 * 86400000;
        let day_end = day_start + 86400000;
        self.events.iter().filter(|e| e.start_ms >= day_start && e.start_ms < day_end).collect()
    }

    pub fn get_events_for_week(&self, week_start_ms: u64) -> Vec<&CalendarEvent> {
        let week_end = week_start_ms + 7 * 86400000;
        self.events.iter().filter(|e| e.start_ms >= week_start_ms && e.start_ms < week_end).collect()
    }

    pub fn get_events_for_month(&self, month_start_ms: u64) -> Vec<&CalendarEvent> {
        let month_end = month_start_ms + 30 * 86400000;
        self.events.iter().filter(|e| e.start_ms >= month_start_ms && e.start_ms < month_end).collect()
    }

    pub fn next_month(&mut self) {
        self.current_date_ms += 30 * 86400000;
    }

    pub fn prev_month(&mut self) {
        self.current_date_ms -= 30 * 86400000;
    }

    pub fn today(&mut self) {
        self.current_date_ms = crate::time::timestamp() as u64;
    }
}
