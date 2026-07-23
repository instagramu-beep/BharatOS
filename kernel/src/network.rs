//! BharatOS kernel network stack — sockets, routing, interfaces
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub const MAX_SOCKETS: usize = 4096;
pub const MAX_ROUTES: usize = 64;

#[derive(Clone, Copy, PartialEq)]
pub enum SocketState { Closed, Bound, Listening, Connected, Closing }

bitflags::bitflags! { pub struct SocketFlags: u32 { const NONBLOCK = 1 << 0; const CLOEXEC = 1 << 1; const REUSEADDR = 1 << 2; } }
bitflags::bitflags! { pub struct NetFlags: u32 { const UP = 1 << 0; const RUNNING = 1 << 1; const LOOPBACK = 1 << 2; const BROADCAST = 1 << 3; } }

#[repr(C)]
pub struct SockAddr { pub family: u16, pub port: u16, pub addr: [u8; 16] }

#[repr(C)]
pub struct Socket {
    pub fd: u32,
    pub domain: u32,
    pub ty: u32,
    pub protocol: u32,
    pub state: SocketState,
    pub local_addr: SockAddr,
    pub remote_addr: SockAddr,
    pub rx_buf: [u8; 65536],
    pub tx_buf: [u8; 65536],
    pub flags: SocketFlags,
}

pub struct NetworkStack {
    pub interfaces: [Option<NetInterface>; 16],
    pub routes: [Route; MAX_ROUTES],
    pub route_count: usize,
    pub sockets: [Option<Socket>; MAX_SOCKETS],
    pub socket_count: usize,
}

#[repr(C)]
pub struct NetInterface {
    pub id: u8,
    pub name: [u8; 16],
    pub mac: [u8; 6],
    pub ipv4: [u8; 4],
    pub mtu: u16,
    pub flags: NetFlags,
    pub state: NetState,
}

#[derive(Clone, Copy, PartialEq)] pub enum NetState { Down, Up, Testing }

#[repr(C)]
pub struct Route { pub dest: [u8; 16], pub mask: [u8; 16], pub gateway: [u8; 16], pub interface: u8, pub metric: u16, pub flags: RouteFlags }
bitflags::bitflags! { pub struct RouteFlags: u32 { const UP = 1 << 0; const GATEWAY = 1 << 1; const DEFAULT = 1 << 4; } }

impl NetworkStack {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }
    pub fn init(&mut self) {
        let _ = self;
    }
    pub fn create_socket(&mut self, domain: u32, ty: u32, protocol: u32) -> Result<u32> {
        if self.socket_count >= MAX_SOCKETS { return Err(crate::err::Error::Full); }
        let sock = Socket {
            fd: self.socket_count as u32, domain, ty, protocol, state: SocketState::Closed,
            local_addr: SockAddr { family: domain as u16, port: 0, addr: [0; 16] },
            remote_addr: SockAddr { family: domain as u16, port: 0, addr: [0; 16] },
            rx_buf: [0; 65536], tx_buf: [0; 65536], flags: SocketFlags::empty(),
        };
        self.sockets[self.socket_count] = Some(sock);
        self.socket_count += 1;
        Ok((self.socket_count - 1) as u32)
    }
    pub fn add_interface(&mut self, iface: NetInterface) -> u8 {
        for i in 0..16 {
            if self.interfaces[i].is_none() { self.interfaces[i] = Some(iface); return i as u8; }
        }
        0xFF
    }
}

static mut NET_STACK: Option<NetworkStack> = None;

pub fn init() { unsafe { NET_STACK = Some(NetworkStack::new()); } if let Some(ref mut net) = NET_STACK { net.init(); } }
pub fn get_stack() -> Option<&'static mut NetworkStack> { unsafe { NET_STACK.as_mut() } }
