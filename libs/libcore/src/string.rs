//! BharatOS libcore string utilities
#![no_std]
#![allow(unused)]

use alloc::string::String;
use alloc::vec::Vec;

impl String {
    pub fn new() -> Self { String::new() }
    pub fn from_bytes(b: &[u8]) -> Self {
        String::from_utf8_lossy(b).into_owned()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes().collect()
    }
    pub fn as_str(&self) -> &str { self }
    pub fn clone_into_vec(&self) -> Vec<u8> {
        self.bytes().collect()
    }
    pub fn push(&mut self, c: char) {
        let mut tmp = [0u8; 4];
        self.push_bytes(c.encode_utf8(&mut tmp).as_bytes());
    }
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        let len = self.len();
        self.reserve(bytes.len());
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.as_mut_ptr().add(len), bytes.len());
            self.set_len(len + bytes.len());
        }
    }
    pub fn contains(&self, pat: &str) -> bool {
        self.find(pat).is_some()
    }
    pub fn starts_with(&self, pat: &str) -> bool {
        self.as_bytes().starts_with(pat.as_bytes())
    }
    pub fn ends_with(&self, pat: &str) -> bool {
        self.as_bytes().ends_with(pat.as_bytes())
    }
    pub fn find(&self, pat: &str) -> Option<usize> {
        self.as_bytes().windows(pat.len()).position(|w| w == pat.as_bytes())
    }
    pub fn split(&self, delim: char) -> Vec<&str> {
        self.split(delim).collect()
    }
    pub fn trim(&self) -> &str {
        self.trim()
    }
    pub fn to_lowercase(&self) -> Self {
        let mut s = String::new();
        for c in self.chars() {
            for b in c.to_lowercase() { s.push(b); }
        }
        s
    }
    pub fn to_uppercase(&self) -> Self {
        let mut s = String::new();
        for c in self.chars() {
            for b in c.to_uppercase() { s.push(b); }
        }
        s
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    pub fn len(&self) -> usize {
        self.len()
    }
    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }
    pub fn clear(&mut self) {
        self.clear();
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        String::from(s)
    }
}

impl From<String> for Vec<u8> {
    fn from(s: String) -> Self {
        s.into_bytes()
    }
}

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn cstring_from_bytes(b: &[u8]) -> Option<String> {
    let null = b.iter().position(|&x| x == 0)?;
    String::from_utf8(b[..null].to_vec()).ok()
}
