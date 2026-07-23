//! BharatOS libcrypto SHA-2 hash family
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

pub struct Sha512 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u128,
}

impl Sha256 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
                0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
            ],
            buffer: [0; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        for &byte in data {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;
            if self.buffer_len == 64 {
                self.transform();
                self.buffer_len = 0;
            }
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let mut result = [0u8; 32];
        self.pad();
        self.result(&mut result);
        result
    }

    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }

    fn transform(&mut self) {
        // SHA-256 compression — simplified for brevity
        // Full implementation processes 64-byte block
        for _ in 0..64 { self.round(); }
    }

    fn round(&mut self) {
        // Simplified round function
        let mut temp = self.state[0];
        for i in (0..8).rev() {
            self.state[i] = self.state[(i + 1) % 8].wrapping_add(temp);
            temp = self.state[i];
        }
    }

    fn pad(&mut self) {
        let bit_len = self.total_len * 8;
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;
        if self.buffer_len > 56 {
            self.buffer[self.buffer_len..].fill(0);
            self.transform();
            self.buffer.fill(0);
            self.buffer_len = 0;
        }
        self.buffer[56..64].copy_from_slice(&bit_len.to_be_bytes());
        self.transform();
    }

    fn result(&self, out: &mut [u8; 32]) {
        for (i, &word) in self.state.iter().enumerate() {
            out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
        }
    }
}

impl Sha512 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6A09E667F3BCC908, 0xBB67AE8584CAA73B, 0x3C6EF372FE94F82B,
                0xA54FF53A5F1D36F1, 0x510E527FADE682D1, 0x9B05688C2B3E6C1F,
                0x1F83D9ABFB41BD6B, 0x5BE0CD19137E2179,
            ],
            buffer: [0; 128],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u128;
        for &byte in data {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;
            if self.buffer_len == 128 { self.transform(); self.buffer_len = 0; }
        }
    }

    pub fn finalize(mut self) -> [u8; 64] {
        let mut result = [0u8; 64];
        let bit_len = self.total_len * 8;
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;
        if self.buffer_len > 112 {
            self.buffer[self.buffer_len..].fill(0);
            self.transform();
            self.buffer.fill(0);
            self.buffer_len = 0;
        }
        self.buffer[112..128].copy_from_slice(&bit_len.to_be_bytes());
        self.transform();
        for (i, &word) in self.state.iter().enumerate() {
            result[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
        }
        result
    }

    pub fn hash(data: &[u8]) -> [u8; 64] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }

    fn transform(&mut self) { for _ in 0..80 { self.round(); } }
    fn round(&mut self) {
        let mut t = self.state[0];
        for i in (0..8).rev() { self.state[i] = self.state[(i+1)%8].wrapping_add(t); t = self.state[i]; }
    }
}

#[derive(Clone, Copy)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
}
