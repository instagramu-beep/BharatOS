//! BharatOS libcore interop — FFI and C ABI compatibility
#![no_std]
#![allow(unused)]

extern crate alloc;
use alloc::ffi::CString;

use crate::prelude::*;

pub struct ForeignPtr<T>(*mut T);

impl<T> ForeignPtr<T> {
    pub fn new(ptr: *mut T) -> Self { Self(ptr) }
    pub fn as_ptr(&self) -> *mut T { self.0 }
    pub fn as_ref(&self) -> Option<&T> { unsafe { self.0.as_ref() } }
    pub fn as_mut(&mut self) -> Option<&mut T> { unsafe { self.0.as_mut() } }
    pub fn null() -> Self { Self(core::ptr::null_mut()) }
    pub fn is_null(&self) -> bool { self.0.is_null() }
}

pub trait IntoCString {
    fn into_c_string(self) -> CString;
}

pub trait FromCString {
    fn from_c_string(s: CString) -> Self;
}
