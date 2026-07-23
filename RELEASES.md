# BharatOS X — Release Notes

## Version 0.1.0 — "Pratima" (Prototype)

**Released: 2025-01-01**

This is the first public prototype of BharatOS X. It is intended for developers,
early adopters, and contributors who want to help build the future of Bharat's
operating system.

### What's New

**Kernel**
- Preemptive multitasking with CFS + RT hybrid scheduler
- 4-level paging (x86_64), buddy allocator, slab allocator
- SMP with per-CPU run queues, NUMA support
- APIC, HPET, PIT, TSC, PIT timers
- IDT with 256 vectors, interrupt handling
- System call interface (syscalls)

**Filesystem**
- BharatFS v1 — journaling, COW, snapshots, compression, encryption
- VFS layer with procfs, devfs, tmpfs

**Hardware**
- x86_64: MSR, PIC, APIC, IDT, serial, PCIe, USB, SATA, NVMe, GPU
- ARM64 and RISC-V boot stubs
- ACPI table discovery, FDT parser

**Desktop**
- Satya Desktop environment
- GPU-accelerated 3D compositor (Vulkan-ready)
- 240 FPS target, HDR, bloom, blur
- Boot diagnostics HUD

**Applications**
- File Manager, Terminal, Editor, Settings
- Image Viewer, Browser, Music, Video, Camera
- Calculator, Calendar, Notes, Task Manager
- Display Manager, PDF Reader, Clock, Sound

**AI**
- BharatAI daemon with offline GGUF model support
- Voice daemon with STT/TTS
- Context memory and pattern learning

**Security**
- Secure Boot chain validation
- Sandboxing and capability model
- Firewall and audit logging
- Full-disk encryption support

**Developer**
- BharatPkg package manager with delta updates
- SDK and debugger
- Comprehensive API documentation

### Known Limitations

- No working network stack yet
- No working audio subsystem yet
- No working USB mass storage yet
- Apps are UI stubs only
- Many drivers are unimplemented

### What's Next

See [ROADMAP.md](ROADMAP.md) for upcoming milestones.

### Download

- ISO: `bharatos-x-0.1.0.iso`
- SHA256: `see releases page`

### Installation

See [README.md](README.md) for installation instructions.
