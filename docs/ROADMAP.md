# BharatOS X — Release Roadmap

## Phase 1: Foundation (Months 1-6)

### Milestone 1: Boot & Kernel Core
- UEFI bootloader with Secure Boot (x86-64)
- Kernel entry with physical memory init
- Virtual memory (4-level paging)
- Early serial console output
- Basic interrupt handling (APIC)
- HPET timer driver
- GDT/IDT setup

### Milestone 2: Memory & Scheduling
- Physical frame allocator (buddy)
- Kernel heap (slab allocator)
- Process creation (fork/clone)
- Initial CFS scheduler
- Timer interrupt → scheduler tick

### Milestone 3: Early Drivers
- PS/2 keyboard + mouse
- Serial port (COM1)
- AHCI SATA driver
- NVMe driver
- EHCI/xHCI USB host
- VESA/VGA framebuffer

### Milestone 4: Filesystem
- BharatFS on-disk format specification
- Block allocator
- Inode table
- Journaling
- Read/write path

## Phase 2: System Services (Months 6-12)

### Milestone 5: VFS & Filesystems
- VFS layer abstraction
- BharatFS driver
- procfs (/proc)
- devfs (/dev)
- tmpfs (/tmp)

### Milestone 6: Networking
- Ethernet driver (e1000/e1000e)
- IPv4 stack
- TCP, UDP
- Socket API
- DHCP client

### Milestone 7: Security
- Capability model
- Sandbox (seccomp-style)
- Ed25519 verification
- Secure Boot chain
- Firewall

### Milestone 8: Init & Services
- Bharat-init (PID 1)
- Service manager
- Logging (unified logger)
- Crash reporting

## Phase 3: Desktop (Months 12-18)

### Milestone 9: Graphics
- Vulkan driver skeleton
- Window system protocol
- Input event routing
- Framebuffer compositing

### Milestone 10: Compositor
- BharatCompositor (Vulkan)
- Window decorations
- Animated wallpapers
- Effects (blur, shadows)

### Milestone 11: Desktop Shell
- Panel, dock, launcher
- App lifecycle management
- Virtual desktops
- Gesture navigation

### Milestone 12: Applications
- File manager
- Terminal emulator
- Settings
- Image viewer
- Calculator, clock, notes

## Phase 4: AI & Polish (Months 18-24)

### Milestone 13: AI Integration
- Voice Daemon (STT/TTS)
- AI Assistant (context memory)
- AI Code Helper
- AI Search

### Milestone 14: Advanced Features
- Browser (AI-powered)
- Office suite
- Photo/video apps
- Gaming support (Vulkan)

### Milestone 15: Developer Tools
- BharatOS SDK
- Package manager
- Debugger
- IDE integration

### Milestone 16: Polish & Release
- Performance optimization
- Accessibility features
- Documentation
- Beta release

## Phase 5: Production (Months 24+)

### Milestone 17: ARM64 & RISC-V
- ARM64 kernel port
- RISC-V kernel port
- Raspberry Pi / SBC support

### Milestone 18: Enterprise
- TPM 2.0 full support
- FDE with TPM sealing
- VPN client
- Enterprise management tools

### Milestone 19: Ecosystem
- App Store
- Third-party SDK adoption
- Driver development kit
- Community build system

### Milestone 20: Global Release
- x86-64, ARM64, RISC-V stable
- LTS support
- Enterprise certifications
- Global community
