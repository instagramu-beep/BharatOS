//! BharatAI daemon — the AI engine of BharatOS — manages voice, text, OCR,
//! image generation, code generation, summarization, translation, TTS, STT,
//! desktop automation, intelligent scoring and online AI integration.
//!
//! BharatAI is designed around a routing engine with:
//!  - Offline models: SST, TTS, intent classification, summarization.
//!  - Client-accelerated model weights: ONNX / GGUF loaded from AppStore.
//!  - Cloud bridge: exchanges requests with BharatAI Cloud (optional).
//!  - Privacy: sends nothing externally without user consent.

#![no_std]
#![allow(unsafe_code)]

pub mod voice_daemon;
pub mod stt;
pub mod tts;
pub mod ocr;
pub mod intent;
pub mod summarizer;
pub mod translator;
pub mod codegen;
pub mod image_gen;
pub mod desktop_automation;
pub mod online_bridge;
pub mod pipeline;
pub mod memory;

use crate::prelude::*;

/// Central BharatAI runtime entry
pub struct BharatAIInstance {
    pub pipeline: pipeline::Pipeline,
    pub voice: voice_daemon::VoiceDaemon,
    pub memory: memory::ContextMemory,
    pub cloud: Option<online_bridge::CloudBridge>,
    pub config: AiConfig,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AiConfig {
    pub model: [u8; 32],            // path to model weights in /var/lib/ai
    pub tts_backend: TtsBackend,
    pub stt_backend: SttBackend,
    pub ocr_engine: OcrEngine,
    pub cloud_enabled: bool,
    pub privacy: PrivacyPolicy,
    pub offline_only: bool,
    pub preferred_language: u32,   // ISO 639-1 code
    pub max_response_length: u16,
}

#[repr(u8)]
pub enum TtsBackend   { GoogleTTS, Heroku, BharatiTTS, VITS }
#[repr(u8)]
pub enum SttBackend   { Whisper, Deepgram, Vosk }
#[repr(u8)]
pub enum OcrEngine    { Tesseract, PaddleOCR }

bitflags::bitflags! {
    pub struct PrivacyPolicy: u32 {
        const NO_LOCAL_AUDIO  = 1 << 0;
        const NO_LOCAL_VISION = 1 << 1;
        const NO_CLOUD       = 1 << 2;
        const NO_HOMEBREW    = 1 << 3;
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model: [0; 32],
            tts_backend: TtsBackend::VITS,
            stt_backend: SttBackend::Whisper,
            ocr_engine: OcrEngine::Tesseract,
            cloud_enabled: false,
            privacy: PrivacyPolicy::all(),
            offline_only: true,
            preferred_language: 9,   // en
            max_response_length: 4096,
        }
    }
}