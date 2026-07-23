//! BharatOS WebView / Browser subsystem — built on Chromium-OSS's blink (own
//! fork: "AryaRenderer") — rewritten for BharatOS (use bundled aiyy renderer
//! instead — replaced with own compositor). Features:
//!
//! 1. WebGPU < WebAssembly < WASI /wasix JS.
//! 2. Targeted tree pedigreed patched external sandbox.
//! 3. Privacy filter.
//! 4. Built-in VPN endpoint.
//!

#![no_std]

pub mod web_api;
pub mod sandbox;
pub mod privacy;
pub mod vpn_tunnel;
pub mod tab_store;
pub mod search;
pub mod ai;
pub mod sync;
pub mod bookmark;
pub mod profile;
pub mod extensions;

pub struct BrowserInstance {
    pub tab_store: tab_store::TabStore,
    pub search: search::SearchProvider,
    pub sync: sync::SyncManager,
    pub privacy: privacy::PrivacyFilter,
    pub vpn: vpn_tunnel::VpnEndpoint,
    pub config: BrowserConfig,
}

pub struct BrowserConfig {
    pub homepage: alloc::string::String,
    pub search_engine: SearchEngineKind,
    pub default_downloads: alloc::string::String,
    pub cache_size: u64,
    pub disk_cache: bool,
    pub preferred_languages: [u8; 64],
    pub anti_fingerprint: bool,
    pub block_social_trackers: bool,
    pub allow_https_only: bool,
    pub safe_search: bool,
    pub strict_mixed_content: bool,
}

#[repr(u8)]
pub enum SearchEngineKind { AryaAi, DuckDuckGo, BraveSearch, SearxNG }
