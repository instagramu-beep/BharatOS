//! BharatOS libcrypto — AES-GCM, SHA2, Ed25519, secure random
#![no_std]
#![allow(unused)]

pub mod aes_gcm;
pub mod ed25519;
pub mod sha2;
pub mod rand;

pub use aes_gcm::AesGcm;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CryptoFlags: u32 { const HARDWARE = 1 << 0; const SECURE = 1 << 1; const FIPS = 1 << 2; }
}

#[repr(C)]
pub struct CryptoEngine {
    pub flags: CryptoFlags,
    pub key: [u8; 32],
    pub initialized: bool,
}

impl CryptoEngine {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(key: [u8; 32]) -> Self {
        let mut engine = Self { flags: CryptoFlags::empty(), key, initialized: true };
        engine.flags.insert(CryptoFlags::SECURE);
        engine
    }
}

static mut CRYPTO_ENGINE: Option<CryptoEngine> = None;

pub fn crypto_init(key: [u8; 32]) -> &'static mut CryptoEngine {
    unsafe {
        CRYPTO_ENGINE = Some(CryptoEngine::init(key));
        CRYPTO_ENGINE.as_mut().unwrap()
    }
}

pub fn get_crypto() -> Option<&'static mut CryptoEngine> {
    unsafe { CRYPTO_ENGINE.as_mut() }
}
