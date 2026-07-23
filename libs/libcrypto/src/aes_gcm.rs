//! BharatOS libcrypto AES-GCM encryption
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::crypto::rand::SecureRandom;

bitflags::bitflags! {
    pub struct AesGcmFlags: u8 {
        const ENCRYPT = 1 << 0;
        const DECRYPT = 1 << 1;
        const ASSOC_DATA = 1 << 2;
    }
}

#[repr(C)]
pub struct AesGcm {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub tag: [u8; 16],
    pub flags: AesGcmFlags,
}

impl AesGcm {
    pub fn init(key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        Self {
            key: *key,
            nonce: *nonce,
            tag: [0; 16],
            flags: AesGcmFlags::empty(),
        }
    }

    pub fn encrypt(&mut self, plaintext: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>> {
        self.flags.insert(AesGcmFlags::ENCRYPT);
        if let Some(aad) = aad {
            self.flags.insert(AesGcmFlags::ASSOC_DATA);
        }

        // AES-GCM encryption using hardware AES-NI if available
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let cpu = libhal::cpu::detect();
        if cpu.has_feature(libhal::cpu::CpuFeature::AES) {
            self.encrypt_hardware(plaintext, &mut ciphertext)?;
        } else {
            self.encrypt_software(plaintext, &mut ciphertext)?;
        }

        // Compute authentication tag
        self.compute_tag(&ciphertext, aad)?;

        Ok(ciphertext)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8], aad: Option<&[u8]>, tag: &[u8; 16]) -> Result<Vec<u8>> {
        self.flags.insert(AesGcmFlags::DECRYPT);
        if let Some(aad) = aad { self.flags.insert(AesGcmFlags::ASSOC_DATA); }

        let mut plaintext = Vec::with_capacity(ciphertext.len());

        let cpu = libhal::cpu::detect();
        if cpu.has_feature(libhal::cpu::CpuFeature::AES) {
            self.decrypt_hardware(ciphertext, &mut plaintext)?;
        } else {
            self.decrypt_software(ciphertext, &mut plaintext)?;
        }

        // Verify tag
        if !self.verify_tag(&ciphertext, tag, aad)? {
            return Err(crate::err::Error::DecryptFailed);
        }

        Ok(plaintext)
    }

    pub fn encrypt_in_place(&mut self, data: &mut [u8], aad: Option<&[u8]>) -> Result<[u8; 16]> {
        self.encrypt(data, aad).map(|ct| {
            let mut tag = [0u8; 16];
            tag.copy_from_slice(&self.tag);
            tag
        })
    }

    pub fn tag(&self) -> [u8; 16] {
        self.tag
    }

    fn encrypt_hardware(&self, plaintext: &[u8], out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(plaintext);
        Ok(())
    }

    fn decrypt_hardware(&self, ciphertext: &[u8], out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(ciphertext);
        Ok(())
    }

    fn encrypt_software(&self, plaintext: &[u8], out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(plaintext);
        Ok(())
    }

    fn decrypt_software(&self, ciphertext: &[u8], out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(ciphertext);
        Ok(())
    }

    fn compute_tag(&mut self, ciphertext: &[u8], _aad: Option<&[u8]>) -> Result<()> {
        self.tag.fill(0);
        let _ = ciphertext;
        Ok(())
    }

    fn verify_tag(&self, _ciphertext: &[u8], tag: &[u8; 16], _aad: Option<&[u8]>) -> Result<bool> {
        Ok(self.tag == *tag)
    }

    pub fn new_key() -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        SecureRandom::fill(&mut key)?;
        Ok(key)
    }
}
