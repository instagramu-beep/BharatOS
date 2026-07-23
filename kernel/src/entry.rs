//! BharatOS kernel entry point — bootstraps the 64-bit environment, initializes
//! hardware abstraction, memory, scheduling, and launches the init process

#![no_std]
#![allow(unused_imports)]

// Core crates (no-STD / kernel-mode)
use libhal::msr;
use libhal::pit;
use libhal::apic;
use libhal::ps2;
use libhal::acpi;
use libhal::usb;
use libhal::pcie;
use libhal::gpu;
use libhal::storage;
use libhal::net;
use libhal::power;

use libmm;
use libsched;
use libfs;
use security;

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// ─── Boot-time globals ────────────────────────────────────────────────────────

static BOOT_PHASE: AtomicU64 = AtomicU64::new(0);    // 0-15: boot stages
static BOOT_ERROR: AtomicU64 = AtomicU64::new(0);
static BOOT_COMPLETE: AtomicBool = AtomicBool::new(false);

// Boot phases for timing/profiling (also used by boot diagnostics overlay)
const PHASE_HAL_INIT: u64           = 1;
const PHASE_MM_INIT: u64            = 2;
const PHASE_ACPI_ENUM: u64         = 3;
const PHASE_INTR_SETUP: u64        = 4;
const PHASE_TIMER_INIT: u64        = 5;
const PHASE_SCHED_INIT: u64        = 6;
const PHASE_VFS_MOUNT: u64         = 7;
const PHASE_DEVICE_ENUM: u64       = 8;
const PHASE_SERVICE_SPAWN: u64     = 9;
const PHASE_DE_INIT: u64           = 10;
const PHASE_USER_SPACE_INIT: u64   = 11;
const PHASE_FULLY_RUNNING: u64     = 12;

// ─── Panic handler ───────────────────────────────────────────────────────────

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    BOOT_ERROR.store(0xDEAD, Ordering::SeqCst);
    security::fatal_panic(info);
    loop { unsafe { core::arch::asm!("hlt") } }
}

// ─── Top of kernel image ─────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn _bharat_kernel_entry() -> ! {
    // Bootloader jumped here with full physical identity mapping
    kernel_log!(Init, "BharatOS kernel entry point reached");

    // Phase 1: early HAL — disable PIC/8259, init IDT, enable protected features
    hal_init();

    // Phase 2: physical memory manager + virtual memory
    mm_init();

    // Phase 3: ACPI + CPU enumeration + SMP bring-up
    acpi_and_cpu_init();

    // Phase 4: APIC, local and I/O, interrupt routing
    intr_init();

    // Phase 5: high-res timer, TSC calibration
    timer_init();

    // Phase 6: scheduler, per-CPU runqueues
    sched_init();

    // Phase 7: VFS, BharatFS mount, procfs, devfs
    fs_init();

    // Phase 8: USB, PCIe, GPU, storage device discovery
    device_discovery();

    // Phase 9: spawn early services (init, logger, cryptod, stateid)
    spawn_early_services();

    // Phase 10: compositor init, display policy, theme
    desktop_subsystem_init();

    // Phase 11: user-space launcher (userspace init process)
    userspace_init();

    BOOT_PHASE.store(PHASE_FULLY_RUNNING, Ordering::SeqCst);
    BOOT_COMPLETE.store(true, Ordering::SeqCst);
    kernel_log!(Ok, "BharatOS kernel fully operational");

    // Enter idle — scheduler is now the controller
    scheduler::idle_loop()
}

// ─── Subsystem inits ─────────────────────────────────────────────────────────

fn hal_init() {
    BOOT_PHASE.store(PHASE_HAL_INIT, Ordering::SeqCst);

    // Architecture-aware HAL init
    arch::init::early_arch_init();

    // Disable and remap legacy PIC (x86)
    libhal::pic::disable_legacy_pic();

    // Init IOAPIC/MSI routing
    libhal::apic::ioapic_init();
    libhal::apic::lapic_init();

    // MSR control — activate EFER.NX, PAT, FS/GS swap
    msr::write(msr::IA32_EFER, msr::read(msr::IA32_EFER) | msr::EFER_NXE | msr::EFER_LMA);
    msr::write(msr::IA32_PAT, msr::PAT_DEFAULT);
    msr::write(msr::IA32_KERNEL_GS_BASE, unsafe { (&gs_base as *const _) as u64 });

    // Init IDT
    {
        use libhal::idt;
        idt::idt_init();
    }

    // Early interrupt vectors
    {
        use libhal::interrupts;
        interrupts::isr_setup_early();
    }

    // PS/2 keyboard and mouse early init
    let _ = ps2::keyboard::init();

    // Boot timing / diagnostic logger
    {
        use libhal::timing;
        timing::boot_timer_init();
    }

    kernel_log!(Init, "HAL initialized");
}

fn mm_init() {
    BOOT_PHASE.store(PHASE_MM_INIT, Ordering::SeqCst);

    // Parse bootloader memory map
    let mmap = libmm::boot_map::parse_boot_memmap();

    // Init physical memory manager (frame allocator)
    libmm::frame::FrameAllocator::init(mmap);

    // Init per-CPU kernel stacks (256 KB each)
    libmm::kernel_stacks::init();

    // Init kernel heap (pre-allocated 256 MB region)
    libmm::heap::init_kernel_heap();

    // Setup paging — higher-half half kernel at 0xFFFFFFFF80000000
    libmm::paging::identity_pages_init();
    libmm::paging::higher_half_map_init();

    kernel_log!(Init, "MM: kernel heap ready");
}

fn acpi_and_cpu_init() {
    BOOT_PHASE.store(PHASE_ACPI_ENUM, Ordering::SeqCst);

    // Parse ACPI tables, enumerate MADT, register local APIC, wake AP cores
    {
        use libhal::acpi;
        boot_log! {
            "ACPI: {} CPUs enumerated",
            {acpi::acpi_init()}
            0  // unused
        }
    }

    // Controller scheduling domains (NUMA if supported)
    {
        use libhal::numa;
        numa::detect_and_build_topology();
    }

    kernel_log!(Init, "CPU topology enumerated, {} cores", arch::cpu_count());
}

fn intr_init() {
    BOOT_PHASE.store(PHASE_INTR_SETUP, Ordering::SeqCst);
    use libhal::interrupts;
    interrupts::apic_irqs_init();
    interrupts::enable_all_vectors();
}

fn timer_init() {
    BOOT_PHASE.store(PHASE_TIMER_INIT, Ordering::SeqCst);
    use libhal::hpet;
    hpet::init_hpet_as_system_clock();

    use libhal::pit;
    pit::disable_pit();

    kernel_log!(Init, "Timer: HPET running");
}

fn sched_init() {
    BOOT_PHASE.store(PHASE_SCHED_INIT, Ordering::SeqCst);
    libsched::init();
    libsched::set_current_priority(64);
}

fn fs_init() {
    BOOT_PHASE.store(PHASE_VFS_MOUNT, Ordering::SeqCst);

    // Virtual filesystem layer
    libfs::vfs::init();

    // Mount BharatFS root
    {
        let root_dev = libhal::storage::get_block_device(0)
            .expect("No boot storage found");
        let bharatfs = libfs::bharatfs::BharatFSFileSystem::new(root_dev, None)
            .expect("BharatFS mount failed");
        libfs::vfs::mount_root(bharatfs);
    }

    // Mount procfs at /proc (no device — virtual)
    libfs::vfs::mount_virtual(procfs::VFSPseudoFS::new(), "/proc\0");

    // Mount devfs at /dev
    libfs::vfs::mount_virtual(devfs::device_fs::new(), "/dev\0");

    // Mount sysfs at /sys
    libfs::vfs::mount_virtual(sysfs::sysfs::new(), "/sys\0");

    kernel_log!(Init, "VFS ready, root mounted");
}

fn device_discovery() {
    BOOT_PHASE.store(PHASE_DEVICE_ENUM, Ordering::SeqCst);

    // PCI/PCIe enumeration — enumerate all endpoints
    libhal::pcie::enumerate_all();

    // USB host controllers
    libhal::usb::hc::init_all_host_controllers();

    // GPU detection and basic init (driver loads later)
    libhal::gpu::detect_gpus();

    // Storage device enumeration
    libhal::storage::enumerate_storage();

    // Network interface enumeration
    libhal::net::net_init();

    // Audio device enumeration
    libhal::audio::detect_devices();

    // Input device enumeration
    libhal::input::enumerate();

    kernel_log!(Init, "Device enumeration complete");
}

fn spawn_early_services() {
    BOOT_PHASE.store(PHASE_SERVICE_SPAWN, Ordering::SeqCst);

    // Spawn required system services
    scheduler::spawn_kthread(bharat_init_path(), None, 32);
    scheduler::spawn_kthread(bharat_logd_path(), None, 20);
    scheduler::spawn_kthread(bharat_cryptod_path(), None, 20);
    scheduler::spawn_kthread(bharat_stateid_path(), None, 40);
    scheduler::spawn_kthread(bharat_netserv_path(), None, 30);
    scheduler::spawn_kthread(bharat_audiod_path(), None, 30);

    kernel_log!(Init, "Early services spawned");
}

fn desktop_subsystem_init() {
    BOOT_PHASE.store(PHASE_DE_INIT, Ordering::SeqCst);
    // Compositor is kernel-graphics + user-mode compositor with shared render context
    // Kernel prepares GPU memory and DRM, then userspace compositor takes over
}

fn userspace_init() {
    BOOT_PHASE.store(PHASE_USER_SPACE_INIT, Ordering::SeqCst);

    // Spawn userland init process — the first userspace process
    // Handles login, display manager orchestration
    let _ = scheduler::spawn_usermode("/sbin/bharat-init");
}

fn scheduler::idle_loop() -> ! {
    loop {
        unsafe {
            arch::cpu::halt_cpu();
            scheduler::schedule();
        }
    }
}

// ─── Helper: CHAR* → &str for kernel paths ─────────────────────────────────────
const fn bharat_init_path() -> *const u8 {
    b"/sbin/bharat-init\0".as_ptr()
}
const fn bharat_logd_path() -> *const u8 {
    b"/sbin/bharat-logd\0".as_ptr()
}
const fn bharat_cryptod_path() -> *const u8 {
    b"/sbin/bharat-cryptod\0".as_ptr()
}
const fn bharat_stateid_path() -> *const u8 {
    b"/sbin/bharat-stateid\0".as_ptr()
}
const fn bharat_netserv_path() -> *const u8 {
    b"/sbin/bharat-netserv\0".as_ptr()
}
const fn bharat_audiod_path() -> *const u8 {
    b"/sbin/bharat-audiod\0".as_ptr()
}

#[no_mangle]
static gs_base: u64 = 0;
