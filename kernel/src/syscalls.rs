//! BharatOS kernel syscalls — user-kernel boundary
#![no_std]
#![allow(unused)]

use crate::prelude::*;
use crate::process::TaskId;

pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_FSTAT: u64 = 5;
pub const SYS_LSEEK: u64 = 8;
pub const SYS_MMAP: u64 = 9;
pub const SYS_MUNMAP: u64 = 11;
pub const SYS_IOCTL: u64 = 16;
pub const SYS_DUP2: u64 = 33;
pub const SYS_NANOSLEEP: u64 = 35;
pub const SYS_GETPID: u64 = 39;
pub const SYS_CLONE: u64 = 56;
pub const SYS_FORK: u64 = 57;
pub const SYS_EXECVE: u64 = 59;
pub const SYS_EXIT: u64 = 60;
pub const SYS_WAIT4: u64 = 61;
pub const SYS_KILL: u64 = 62;
pub const SYS_GETTIMEOFDAY: u64 = 96;
pub const SYS_SOCKET: u64 = 41;
pub const SYS_BIND: u64 = 49;
pub const SYS_LISTEN: u64 = 50;
pub const SYS_ACCEPT: u64 = 43;
pub const SYS_CONNECT: u64 = 42;
pub const SYS_SENDTO: u64 = 44;
pub const SYS_RECVFROM: u64 = 45;
pub const SYS_BRK: u64 = 12;

bitflags::bitflags! {
    pub struct OpenFlags: u32 {
        const O_RDONLY = 0;
        const O_WRONLY = 1 << 0;
        const O_RDWR = 1 << 1;
        const O_CREAT = 1 << 6;
        const O_EXCL = 1 << 7;
        const O_TRUNC = 1 << 9;
        const O_APPEND = 1 << 10;
        const O_NONBLOCK = 1 << 11;
        const O_DIRECTORY = 1 << 16;
        const O_NOFOLLOW = 1 << 17;
    }
}

bitflags::bitflags! {
    pub struct MmapProt: u32 {
        const PROT_READ = 1 << 0;
        const PROT_WRITE = 1 << 1;
        const PROT_EXEC = 1 << 2;
    }
}

bitflags::bitflags! {
    pub struct MmapFlags: u32 {
        const MAP_SHARED = 1 << 0;
        const MAP_PRIVATE = 1 << 1;
        const MAP_ANONYMOUS = 1 << 5;
        const MAP_FIXED = 1 << 4;
        const MAP_STACK = 0x20;
    }
}

#[repr(C)]
pub struct SyscallArgs {
    pub nr: u64,
    pub args: [u64; 6],
}

pub struct SyscallResult {
    pub value: i64,
    pub error: bool,
}

impl SyscallResult {
    pub fn ok(val: i64) -> Self { Self { value: val, error: false } }
    pub fn err(code: i64) -> Self { Self { value: code, error: true } }
}

pub fn dispatch(args: &SyscallArgs) -> SyscallResult {
    match args.nr {
        SYS_READ => sys_read(args.args[0], args.args[1] as *mut u8, args.args[2]),
        SYS_WRITE => sys_write(args.args[0], args.args[1] as *const u8, args.args[2]),
        SYS_OPEN => sys_open(args.args[0] as *const u8, args.args[1] as u32, args.args[2] as u32),
        SYS_CLOSE => sys_close(args.args[0]),
        SYS_GETPID => SyscallResult::ok(process::current_pid() as i64),
        SYS_FORK => sys_fork(),
        SYS_EXECVE => sys_execve(args.args[0] as *const u8, args.args[1] as *const *const u8, args.args[2] as *const *const u8),
        SYS_EXIT => sys_exit(args.args[0] as i32),
        SYS_KILL => sys_kill(args.args[0] as u32, args.args[1] as u32),
        SYS_MMAP => sys_mmap(args.args[0], args.args[1], args.args[2] as u32, args.args[3] as u32, args.args[4] as u32, args.args[5]),
        SYS_SOCKET => sys_socket(args.args[0] as u32, args.args[1] as u32, args.args[2] as u32),
        _ => SyscallResult::err(-1),
    }
}

fn sys_read(fd: u64, buf: *mut u8, len: u64) -> SyscallResult {
    let _ = (fd, buf, len);
    SyscallResult::err(-38) // ENOSYS
}

fn sys_write(fd: u64, buf: *const u8, len: u64) -> SyscallResult {
    let _ = (fd, buf, len);
    SyscallResult::err(-38)
}

fn sys_open(path: *const u8, flags: u32, mode: u32) -> SyscallResult {
    let _ = (path, flags, mode);
    SyscallResult::err(-38)
}

fn sys_close(fd: u64) -> SyscallResult {
    let _ = fd;
    SyscallResult::err(-38)
}

fn sys_fork() -> SyscallResult {
    SyscallResult::err(-38)
}

fn sys_execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> SyscallResult {
    let _ = (path, argv, envp);
    SyscallResult::err(-38)
}

fn sys_exit(code: i32) -> SyscallResult {
    let _ = code;
    SyscallResult::err(-38)
}

fn sys_kill(pid: u32, sig: u32) -> SyscallResult {
    let _ = (pid, sig);
    SyscallResult::err(-38)
}

fn sys_mmap(addr: u64, len: u64, prot: u32, flags: u32, fd: u32, off: u64) -> SyscallResult {
    let _ = (addr, len, prot, flags, fd, off);
    SyscallResult::err(-38)
}

fn sys_socket(domain: u32, ty: u32, protocol: u32) -> SyscallResult {
    let _ = (domain, ty, protocol);
    SyscallResult::err(-38)
}
