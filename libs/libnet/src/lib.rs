//! BharatOS libnet — TCP/IPv4/IPv6/UDP/TLS networking stack
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod socket;
pub mod tcp;
pub mod udp;
pub mod ipv4;
pub mod ipv6;
pub mod dns;
pub mod dhcp;
pub mod tls;
pub mod http;
pub mod websocket;
pub mod netlink;

pub use socket::*;
pub use tcp::*;
pub use udp::*;
pub use ipv4::*;
pub use ipv6::*;

pub const MAX_SOCKETS: usize = 4096;
pub const MAX_ROUTES: usize = 64;
pub const MAX_INTERFACES: usize = 16;

bitflags::bitflags! {
    pub struct NetFlags: u32 {
        const UP = 1 << 0;
        const RUNNING = 1 << 1;
        const LOOPBACK = 1 << 2;
        const BROADCAST = 1 << 3;
        const MULTICAST = 1 << 4;
        const PROMISC = 1 << 5;
    }
}
