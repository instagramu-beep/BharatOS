#![no_std]
#![no_main]
#![allow(unused)]

mod boot_params;
mod arch_detect;
mod memory_map;
mod config;
mod loader;
mod secure_boot;
mod efi_runtime;
mod graphics_switch;
mod chainload;

use core::sync::atomic::{AtomicU64, Ordering};
use uefi::prelude::*;
use uefi::boot::{MemoryType, MemoryMapKey, MemoryAttribute, ScopedProtocol, LoadImageSource};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::{MemoryDescriptor, SearchType};
use uefi::guid;
use boot_params::BootParameters;
use memory_map::MemoryMap;
use config::BootConfig;
use secure_boot::SecureBootValidator;
use graphics_switch::GraphicsMode;

/// Unique UEFI boot marker GUID for OS detection
const BHARAT_BOOT_GUID: uefi::guid::Guid =
    uefi::guid!(0x4B4F5241, 0x4B48, 0x4F53, 0x594F532D); // "OBHIS-OS"

/// Boot session ID for debugging and crash dumps
static BOOT_SESSION_ID: AtomicU64 = AtomicU64::new(0);

#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) -> Status {
    // Kill watchdog — we don't want reboots during boot
    let _ = st.boot_services().set_watchdog_timer(0, 0x10000, None);

    // Generate session ID for this boot
    BOOT_SESSION_ID.store(
        ((st.uefi_firmware_revision() as u64) << 32)
            | (st.firmware_vendor().len() as u64),
        Ordering::Relaxed,
    );

    let boot_services = st.boot_services();

    // Resolve kernel ELF file via firmware protocol or fallback media
    let kernel_image = match load_kernel_image(boot_services) {
        Ok(img) => img,
        Err(e) => {
            uefi::helpers::panic(e);
        }
    };

    // Query and build memory map
    let mmap = query_memory_map(boot_services);

    // Detect processor architecture and configuration
    let arch = arch_detect::detect_architecture(st.runtime_services());

    // Detect available ACPI tables
    let acpi = arch_detect::detect_acpi(st.runtime_services(), &mmap);

    // Secure Boot verification (if enabled)
    let secure_boot = SecureBootValidator::new(st.runtime_services());
    if secure_boot.is_enabled() {
        if let Err(e) = secure_boot.verify_kernel(&kernel_image) {
            st.stdout().write_str("Secure Boot verification failed: ").ok();
            st.stdout().write_str(e).ok();
            st.boot_services().stall(3_000_000);
            return Status::SECURITY_VIOLATION;
        }
        st.stdout().write_str("[Secure Boot] Kernel signature verified\n").ok();
    }

    // Read kernel boot parameters
    let boot_params = BootParameters::load(boot_services);

    // Initialize kernel ELF loader
    let (kernel_entry, phys_load_base) = match loader::load_kernel_elf(
        boot_services,
        &kernel_image,
        &mmap,
        arch.page_size(),
    ) {
        Ok(data) => data,
        Err(e) => uefi::helpers::panic(alloc::format!("Failed to load kernel ELF: {}", e)),
    };

    // Set up GOP framebuffer for passing to kernel (VESA/FrameBuffer info)
    let gop = graphics_switch::acquire_framebuffer(boot_services);
    let frame = GraphicsMode::from_gop(gop.as_ref());

    // Enable interrupts if UEFI chose WHV mode
    st.runtime_services().set_virtual_address_map(
        mmap.key().clone(),
        mmap.descriptor_size() as usize,
        mmap.descriptor_version(),
        arch.page_size() as usize,
    ).unwrap_or_else(|_| {
        st.stderr().write_str("Warning: failed to switch VADT\n").ok();
    });

    // Build final boot args struct in low memory (below 4 GB)
    let boot_info_addr: u64 = 0x100; // Fixed physical allocation below 1 MB
    (boot_info_addr as *mut BootInfo).write_volatile(BootInfo {
        magic: 0x42484F53545F4F53u64, // "BHARAT_OS"
        version: 1,
        session_id: BOOT_SESSION_ID.load(Ordering::Relaxed),
        memory_map: mmap.into_phys_addr(),
        memory_map_size: mmap.len() * mmap.descriptor_size(),
        descriptor_size: mmap.descriptor_size() as u32,
        fps: frame.width,
        fpb: frame.pitch,
        fh: frame.height,
        framebuffer: frame.base,
        acpi_root: acpi.rsdt as u64,
        cpu_count: arch.cpu_count(),
        cpu_features: arch.cpu_features(),
        secure_boot_enabled: secure_boot.is_enabled(),
        page_size: arch.page_size() as u32,
    });

    // Notify EFI we're handing off control
    st.boot_services().exit_boot_services(mmap.key()).expect("ExitBootServices failed");

    unsafe {
        // Jump to kernel — naked asm with canonical registers
        core::arch::asm!(
            "jmp {entry}",
            entry = in(reg) kernel_entry,
            options(noreturn, att_syntax),
        );
    }
}

// ─── BootInfo struct — provides kernel with hardware map ──────────────────────
#[repr(C, packed)]
struct BootInfo {
    magic: u64,
    version: u32,
    session_id: u64,
    memory_map: u64,
    memory_map_size: u32,
    descriptor_size: u32,
    fps: u32,       // framebuffer width
    fh: u32,        // framebuffer height
    fpb: u32,       // pitch (bytes per row)
    framebuffer: u64,
    acpi_root: u64,
    cpu_count: u32,
    cpu_features: u64,
    secure_boot_enabled: u32,
    page_size: u32,
}

// ─── Image Resolution ─────────────────────────────────────────────────────────
fn load_kernel_image(bs: &BootServices) -> Result<ScopedProtocol<LoadedImage>, &'static str> {
    // Try firmware MEDIA_FILE_PATH on the boot handle first
    let boot_handle = bs.image_handle();

    // Try to load predefined kernel path
    let path = uefi::cstr16!("/\\EFI\\BharatOS\\kernel.elf");
    match bs.load_image(boot_handle, LoadImageSource::FromDevice {
        device_path: path,
        file_path: None,
    }) {
        Ok(img) => return Ok(unsafe { ScopedProtocol::<LoadedImage>::from_handle(img) }),
        Err(_) => {}
    }

    // Fallback: search removable media recursively
    let mut handles =
        match bs.find_handles::<uefi::proto::media::file::File>() {
            Ok(h) => h,
            Err(_) => return Err("No filesystems found"),
        };

    for handle in handles {
        if let Ok(file_system) = bs.handle_protocol::<uefi::proto::media::fs::SimpleFileSystem>(handle) {
            let fs = unsafe { &*file_system.get() };
            if let Ok(mut root) = fs.open_volume() {
                if find_and_load_elf(bs, &mut root, uefi::cstr16!("/")).is_ok() {
                    break; // Loaded
                }
            }
        }
    }

    Err("Kernel ELF not found on any accessible volume")
}

fn find_and_load_elf(
    bs: &BootServices,
    dir: &mut uefi::proto::media::file::Directory,
    prefix: &uefi::CString16,
) -> Result<ScopedProtocol<LoadedImage>, &'static str> {
    use uefi::proto::media::file::{File, FileMode};
    let mut buf = [0u8; 4096];
    for entry in dir.iter_entries(&mut buf) {
        let entry = entry.map_err(|_| "readdir")?;
        let name = entry.file_name();
        let path = format!("{}{}", prefix, name.to_string().unwrap_or_default());

        if entry.is_directory() { continue; }
        let candidate = alloc::format!("{}kernel.elf", path);
        let cpath = uefi::CString16::try_from(candidate.as_str()).or(uefi::CStr16::from(prefix)).map_err(|_| "path")?;
        match bs.load_image(
            bs.image_handle(),
            LoadImageSource::FromDevice {
                device_path: &cpath,
                file_path: None,
            },
        ) {
            Ok(h) => return Ok(unsafe { ScopedProtocol::<LoadedImage>::from_handle(h) }),
            Err(_) => {}
        }
    }
    Err("no elf")
}

// ─── Memory Map ──────────────────────────────────────────────────────────────
fn query_memory_map(bs: &BootServices) -> MemoryMap {
    let mut buf = [0u8; 0x20000];
    let (_key, desc_iter) = bs.memory_map(&mut buf).expect("memory_map");
    MemoryMap::new(desc_iter)
}
