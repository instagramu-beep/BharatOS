//! BharatOS init — systemd-style init process
#![no_std]
#![allow(unused)]

pub mod service;
pub mod target;
pub mod socket;
pub mod mount;
pub mod device;
pub mod timer;
pub mod path;
pub mod scope;
pub mod slice;
