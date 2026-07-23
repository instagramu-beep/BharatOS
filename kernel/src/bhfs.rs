//! BharatOS kernel BharatFS integration (re-export from libfs)
#![no_std]
#![allow(unused)]

pub use crate::fs::bhfs::*;
pub use crate::fs::vfs::*;

pub trait BharatFSFilesystem: VfsFilesystem {
    fn create_snapshot(&self, path: &str, name: &str) -> Result<u64>;
    fn restore_snapshot(&self, path: &str, snap_idx: u64) -> Result<()>;
    fn list_snapshots(&self, path: &str) -> Result<Vec<super::fs::bhfs::Snapshot>>;
    fn enable_compression(&self, path: &str) -> Result<()>;
    fn set_encryption(&self, path: &str, key: &[u8]) -> Result<()>;
}
