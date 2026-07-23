//! BharatOS kernel tmpfs — in-memory temporary filesystem
#![no_std]
#![allow(unused)]

pub use crate::fs::tmpfs::*;

pub struct TmpFsManager {
    pub root: TmpFsNode,
    pub used_bytes: u64,
    pub max_bytes: u64,
}

#[derive(Clone, Copy)]
pub struct TmpFsNode {
    pub id: u64,
    pub name: [u8; 256],
    pub ty: VfsNodeType,
    pub size: u64,
    pub mode: u32,
    pub children: Vec<TmpFsNode>,
    pub data: [u8; 4096],
    pub data_len: usize,
}

impl TmpFsManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn init(&mut self) {
        self.root = TmpFsNode {
            id: 2,
            name: *b"/",
            ty: VfsNodeType::Directory,
            size: 0,
            mode: 0o755,
            children: Vec::new(),
            data: [0; 4096],
            data_len: 0,
        };
        self.used_bytes = 0;
        self.max_bytes = 256 * 1024 * 1024;
    }

    pub fn lookup(&self, path: &str) -> Option<&TmpFsNode> {
        let mut node = &self.root;
        for component in path.split('/').filter(|c| !c.is_empty()) {
            if let Some(child) = node.children.iter().find(|c| c.name.starts_with(component.as_bytes())) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }

    pub fn mkdir(&mut self, path: &str) -> Result<&TmpFsNode> {
        let components: Vec<&str> = path.split('/').filter(|c| !c.is_empty()).collect();
        if components.is_empty() {
            return Ok(&mut self.root);
        }
        let mut node = &mut self.root;
        for component in components {
            let idx = node.children.iter().position(|c| c.name.starts_with(component.as_bytes()));
            match idx {
                Some(i) => node = &mut node.children[i],
                None => {
                    let new_node = TmpFsNode {
                        id: Self::next_id(),
                        name: name_to_array(component),
                        ty: VfsNodeType::Directory,
                        size: 0,
                        mode: 0o755,
                        children: Vec::new(),
                        data: [0; 4096],
                        data_len: 0,
                    };
                    node.children.push(new_node);
                    node = node.children.last_mut().unwrap();
                }
            }
        }
        Ok(node)
    }

    pub fn get_usage(&self) -> TmpFsUsage {
        TmpFsUsage {
            used_bytes: self.used_bytes,
            max_bytes: self.max_bytes,
            used_percent: (self.used_bytes * 100 / self.max_bytes) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TmpFsUsage {
    pub used_bytes: u64,
    pub max_bytes: u64,
    pub used_percent: u8,
}

static mut TMPFS_MANAGER: Option<TmpFsManager> = None;

pub fn init() {
    unsafe { TMPFS_MANAGER = Some(TmpFsManager::new()); }
    if let Some(ref mut mgr) = TMPFS_MANAGER { mgr.init(); }
}

pub fn mount() -> Result<()> {
    init();
    Ok(())
}

pub fn lookup(path: &str) -> Option<TmpFsNode> {
    unsafe { TMPFS_MANAGER.as_ref().and_then(|mgr| mgr.lookup(path).cloned()) }
}

fn name_to_array(s: &str) -> [u8; 256] {
    let mut buf = [0u8; 256];
    let bytes = s.as_bytes();
    let len = bytes.len().min(255);
    buf[..len].copy_from_slice(&bytes[..len]);
    buf
}

static mut NEXT_TMPFS_ID: u64 = 3;

impl TmpFsManager {
    fn next_id() -> u64 {
        unsafe {
            let id = NEXT_TMPFS_ID;
            NEXT_TMPFS_ID = NEXT_TMPFS_ID.wrapping_add(1);
            id
        }
    }
}
