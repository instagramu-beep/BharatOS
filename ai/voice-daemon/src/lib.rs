//! BharatOS voice daemon — STT/TTS/voice interaction
#![no_std]
#![allow(unused)]

pub mod stt;
pub mod tts;
pub mod wake_word;
pub mod vad;
pub mod aec;
pub mod ns;
pub mod agc;

bitflags::bitflags! {
    pub struct VoiceFlags: u32 {
        const LISTENING = 1 << 0;
        const SPEAKING = 1 << 1;
        const WAKE_WORD_DETECTED = 1 << 2;
        const PROCESSING = 1 << 3;
        const MUTED = 1 << 4;
    }
}
