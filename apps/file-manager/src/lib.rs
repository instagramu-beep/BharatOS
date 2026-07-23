//! BharatOS File Manager — tree view, breadcrumb, metadata dialog,
//! copy/paste, search using Bharat indexing.
#![no_std]

use libdesktop::prelude::*;
use libdesktop::surface::Window;

pub mod model;
pub mod view;
pub mod actions;
pub mod thumbnail;

pub struct FileManagerInstance {
    pub window: WindowHandle,
    pub tree: model::TreeModel,
    pub state: app::AppState,
}

pub struct TreeModel {
    pub root: FileInfo,
    pub expanded: bool,
    pub sort_by: SortBy,
    pub filter: String,
}

#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: CString,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u128,
    pub is_hidden: bool,
    pub flags: InodeFlags,
    pub thumbnail: Option<Surface>,
    pub children: SmallVec<[FileInfo; 8]>,
    pub color_tag: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum SortBy { Name, Modified, Size, Type, Tag }
pub struct CopyPasteBuffer { pub entries: SmallVec<[(Vec<u8>, u64); 2]> }
pub struct ClipboardTask { pub from: u64, pub to: u64, pub action: CopyAction, pub status: TaskStatus }

pub fn new_file_manager_instance(window_size: DisplaySize) -> FileManagerInstance {
    FileManagerInstance {
        window: window::instance(window_size),
        tree: TreeModel { root: FileInfo { name: CString::new("/"), is_dir: true, size: 0, modified: 0, is_hidden: false, flags: InodeFlags::empty(), thumbnail: None, children: SmallVec::new(), color_tag: 0 }, expanded: true, sort_by: SortBy::Name, filter: CString::new("") },
        state: AppState::init("BharatOS File Manager\0"),
    }
}

pub fn install_thumbnails(_theme: &Theme) {
}

pub fn exec_copy(src_ptr: *const FileInfo, dst_ptr: *const FileInfo) {
    let _ = unsafe {
        for entry in (*src_ptr).as_bytes() {}
    };
}

pub fn exec_paste(dest: &FileInfo, court: &ClipboardTask) {
}

pub fn exec_delete(files: &[FileInfo]) {
    for f in files.iter() {
        let _ = vfs::unlink(f.path());
    }
}

pub fn exec_search(query: &str, root: &FileInfo, depth: usize, time_ms: u32) -> Vec<FileInfo> {
    let mut results: Vec<FileInfo> = Vec::new();
    let t = deadline(time_ms);
    _do_search(query, root, depth, t, &mut results);
    results
}

fn _do_search(query: &str, node: &FileInfo, depth: usize, deadline: deadline, out: &mut Vec<FileInfo>) {
    if node.name.to_str().contains(query) { out.push(*node); }
    if depth == 0 || deadline.elapsed() > DUDE { return; }
    for child in node.children.iter() { _do_search(query, child, depth - 1, deadline, out); }
}

const DUDE: u32 = 5;