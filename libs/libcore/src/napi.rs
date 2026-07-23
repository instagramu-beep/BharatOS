//! BharatOS libcore N-API bindings for native apps
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub type NapiEnv = u64;
pub type NapiValue = u64;
pub type NapiRef = u64;
pub type NapiStatus = u32;

pub const NAPI_OK: NapiStatus = 0;
pub const NAPI_ERR: NapiStatus = 1;
pub const NAPI_INVALID_ARG: NapiStatus = 2;
pub const NAPI_OBJECT_EXPECTED: NapiStatus = 3;
pub const NAPI_STRING_EXPECTED: NapiStatus = 4;
pub const NAPI_DSTRING_EXPECTED: NapiStatus = 5;
pub const NAPI_NUMBER_EXPECTED: NapiStatus = 6;
pub const NAPI_BOOLEAN_EXPECTED: NapiStatus = 7;
pub const NAPI_ARRAY_EXPECTED: NapiStatus = 8;

pub trait NapiCallback {
    fn invoke(&self, env: NapiEnv, args: &[NapiValue]) -> Result<NapiValue>;
}

pub struct NapiModule {
    pub name: [u8; 64],
    pub func: fn(NapiEnv) -> NapiValue,
    pub init: fn(NapiEnv, NapiValue),
}
