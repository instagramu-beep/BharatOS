//! BharatOS AI memory and context system — manages conversation history,
//! user preferences, learned patterns, and contextual awareness.
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod context;
pub mod session;
pub mod memory_graph;
pub mod preference_engine;
pub mod pattern_learner;

pub struct AiMemorySystem {
    pub sessions: [Option<AiSession>; 64],
    pub session_count: usize,
    pub global_context: GlobalContext,
    pub preference_engine: preference_engine::PreferenceEngine,
    pub pattern_learner: pattern_learner::PatternLearner,
    pub memory_graph: memory_graph::MemoryGraph,
}

#[derive(Clone, Copy)]
pub struct AiSession {
    pub id: u64,
    pub start_time: u128,
    pub last_activity: u128,
    pub context: context::ConversationContext,
    pub user_id: u32,
    pub app_context: AppContext,
    pub max_history: u32,
}

#[derive(Clone, Copy)]
pub struct GlobalContext {
    pub current_user: u32,
    pub active_apps: [u32; 8],
    pub system_time: u128,
    pub location: Option<GeoLocation>,
    pub language: u32,
    pub accessibility_prefs: AccessibilityPrefs,
    pub power_profile: u8,
    pub network_quality: u8,
}

#[derive(Clone, Copy)]
pub struct AppContext {
    pub app_id: u32,
    pub window_handle: u64,
    pub active_document: Option<[u8; 128]>,
    pub selection: Option<TextRange>,
    pub clipboard_content: Option<[u8; 256]>,
}

#[derive(Clone, Copy)]
pub struct TextRange { pub start: u32, pub end: u32, pub line: u32 }

#[derive(Clone, Copy)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_m: f32,
}

#[derive(Clone, Copy)]
pub struct AccessibilityPrefs {
    pub screen_reader_enabled: bool,
    pub high_contrast: bool,
    pub large_text: bool,
    pub reduced_motion: bool,
    pub voice_control: bool,
}

impl AiMemorySystem {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn start_session(&mut self, user_id: u32) -> &mut AiSession {
        let id = timestamp() as u64;
        let session = AiSession {
            id,
            start_time: timestamp(),
            last_activity: timestamp(),
            context: context::ConversationContext::new(),
            user_id,
            app_context: AppContext::default(),
            max_history: 100,
        };
        let idx = self.session_count % 64;
        self.sessions[idx] = Some(session);
        self.session_count += 1;
        self.sessions[idx].as_mut().unwrap()
    }

    pub fn get_active_session(&self) -> Option<&AiSession> {
        self.sessions.iter().find_map(|s| s.as_ref())
    }

    pub fn update_context(&mut self, ctx: context::ConversationContext) {
        if let Some(session) = self.get_active_session_mut() {
            session.context = ctx;
            session.last_activity = timestamp();
        }
    }

    pub fn learn_pattern(&mut self, pattern: &UserPattern) {
        self.pattern_learner.ingest(pattern);
    }

    pub fn recall(&self, query: &str) -> Vec<MemoryFragment> {
        self.memory_graph.search(query)
    }
}
