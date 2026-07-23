# BharatOS X

** Bharat (India) — Independent, Original Operating System **

BharatOS X is a completely original, next-generation operating system designed and built from scratch in Bharat. It is not based on Linux, Windows, macOS, Android, BSD, or any existing operating system.

## Vision

Create a modern, AI-first operating system developed in Bharat that prioritizes performance, security, privacy, accessibility, and an exceptional user experience with a GPU-accelerated 3D desktop.

## Core Technologies

### Custom Kernel (BharatOS Kernel)
- **Preemptive multitasking** with CFS+RT hybrid scheduler
- **Multi-threading** and multi-core CPU scheduling with NUMA support
- **Symmetric multiprocessing** (SMP) with per-CPU run queues
- **Virtual memory management** with 4-level paging (x86-64)
- **Physical memory manager** — buddy allocator + slab caches
- **Process manager** with fork/clone/exec model
- **Thread scheduler** — fair scheduling for normal, RT for critical
- **Interrupt handling** — APIC, MSI, MSI-X support
- **Hardware abstraction layer** (libhal)
- **Device manager** with driver framework
- **Power management** — CPU frequency scaling, display power, battery
- **Filesystem manager** — BharatFS journaling filesystem
- **IPC system** — pipes, sockets, shared memory, signals
- **Security subsystem** — sandbox, firewall, capability model
- **Network stack** — TCP/IPv4/IPv6/UDP/TLS 1.3
- **Graphics subsystem** — Vulkan-compatible renderer
- **Audio subsystem** — HDA-compatible audio pipeline
- **USB subsystem** — OHCI/UHCI/EHCI/xHCI
- **Bluetooth stack** — BLE + Classic Bluetooth
- **Wi-Fi stack** — 802.11a/b/g/n/ac/ax
- **PCI support** — full PCIe enumeration
- **ACPI support** — power management, device enumeration
- **UEFI boot** with Secure Boot and Fast Boot
- **Multi-arch**: x86-64, ARM64, RISC-V

### Boot System
- UEFI-native bootloader
- GPT partition support
- Secure Boot verification
- Fast Boot (< 5 seconds on supported hardware)
- Boot recovery and diagnostics
- Boot encryption support
- Multi-OS boot manager

### Custom Filesystem (BharatFS)
- **Journaling** for data integrity
- **Snapshots** for instant rollback
- **Transparent compression** (LZ4/ZSTD)
- **Deduplication** for storage efficiency
- **Full-disk encryption** (AES-256-GCM, ChaCha20-Poly1305)
- **Checksums** for every block
- **Self-healing** from corruption
- **Copy-on-write** for snapshots and safe updates
- **Large file support** (> 16 TB)
- **SSD optimization** — TRIM support
- **Fast indexing** for instant search
- **Instant search** API

### Desktop Environment (Satya Desktop)
- Full GPU-accelerated 3D compositor
- Real-time lighting effects
- Animated and live 3D wallpapers
- Interactive desktop objects
- Transparent windows with glass/blur effects
- Dynamic shadows and reflections
- Weather and Earth wallpapers
- Particle effects and smooth 240 FPS animations
- Virtual desktops and gesture navigation
- AI-powered desktop organization
- Custom widgets and smart dock
- Voice interaction

### User Interface
- Rounded windows with Material-inspired design
- Dynamic themes — Light/Dark/Auto
- Fluid animations
- High DPI, multi-monitor, touchscreen, pen support
- Keyboard shortcuts and voice commands

### AI Integration (BharatAI)
- Voice and text conversations
- File search with semantic understanding
- Image generation
- Document summarization and translation
- Code generation and assistance
- Email and calendar management
- OCR, STT, TTS
- Desktop automation
- Offline AI models with optional cloud integration

### First-Party Applications
- **Bharat Browser** — AI search, privacy mode, ad/tracker blocking
- **File Manager** — BharatFS-native with thumbnails
- **Terminal** — GPU-accelerated, PTY, AI assistance
- **Office Suite** — Word, Spreadsheet, Presentation
- **Notes**, **Drawing**, **PDF Reader**
- **Music Player**, **Video Player**, **Camera**
- **Calculator**, **Clock**, **Calendar**
- **Settings**, **Task Manager**, **System Monitor**
- **AI Studio**, **IDE**, **App Store**
- **Backup**, **Disk Utility**, **Archive Manager**

### Security
- Secure Boot with verified kernel
- Full-disk encryption (FDE)
- Sandboxed applications
- Verified updates (atomic, delta, rollback)
- Firewall and malware protection
- Permission manager with capability model
- Password manager, face recognition, fingerprint authentication
- Zero-trust principles
- Secure networking with TLS 1.3

### Performance
- Boot time under 5 seconds on supported hardware
- Low RAM usage (< 512 MB idle)
- Efficient CPU scheduling
- Fast application launch
- Low battery consumption
- GPU acceleration throughout
- Fast file indexing
- High responsiveness (240 FPS target)

### Game Support
- Vulkan native
- DirectX compatibility layer
- OpenGL support
- High refresh rate support
- Game mode optimization
- Controller support

### Developer Platform
- BharatOS SDK
- Native compiler (BharatCC)
- Package manager (BharatPkg)
- Debugger with kernel/userspace support
- IDE integration
- Emulator for testing
- API documentation
- Plugin framework
- Git integration
- AI coding assistant

### Accessibility
- Screen reader
- Voice control
- Magnifier
- High contrast mode
- Keyboard-only navigation
- Captions and color filters
- Large text mode

### Updates
- Atomic updates with rollback
- Delta updates for minimal downloads
- Automatic updates
- Offline update support
- Update verification

### Hardware Support
- Architecture: x86-64, ARM64, RISC-V
- Graphics: Intel, AMD, NVIDIA (Vulkan-capable)
- Storage: SATA, NVMe, USB Mass Storage
- Networking: Ethernet, Wi-Fi 6/6E/7, Bluetooth 5.x
- Audio: HDA-compatible codecs
- Input: Keyboard, mouse, touch, pen, controllers
- Peripherals: Printers, webcams, scanners

## Repository Structure

```
bharatos/
├── boot/                    # UEFI bootloader
│   ├── src/main.rs
│   └── Cargo.toml
├── kernel/
│   ├── src/
│   │   ├── entry.rs         # Kernel entry point
│   │   ├── arch.rs          # Architecture abstraction
│   │   ├── timer.rs         # PIT/HPET timers
│   │   ├── power.rs         # Power management
│   │   ├── process.rs       # Process management
│   │   ├── logger.rs        # Unified logging
│   │   └── virtual.rs       # Virtualization support
│   └── Cargo.toml
├── libs/
│   ├── libcore/             # Core no_std library
│   ├── libhal/              # Hardware Abstraction Layer
│   │   ├── src/msr.rs       # Model Specific Registers
│   │   ├── src/pic.rs       # PIC controller
│   │   ├── src/apic.rs      # APIC controller
│   │   └── src/idt.rs       # Interrupt Descriptor Table
│   ├── libmm/               # Memory management
│   ├── libsched/            # Scheduler
│   │   └── src/runqueue.rs  # Run queue implementations
│   ├── libfs/               # Virtual filesystem
│   ├── libnet/              # Networking
│   ├── libaudio/            # Audio subsystem
│   ├── libinput/            # Input devices
│   ├── libcrypto/           # Cryptography
│   ├── libaep/              # AI Enhancement Platform
│   └── libsurface/          # GPU surface API
├── fs/
│   ├── bharatfs/            # BharatFS filesystem
│   ├── mount/               # Mount utility
│   └── mkfs-bharatfs/       # Format utility
├── sbin/
│   ├── bharat-init/         # Init process
│   ├── bharat-pkg/          # Package manager
│   ├── bharat-fsck/         # Filesystem checker
│   └── bharat-cryptsetup/   # Disk encryption
├── desktop/
│   ├── bharat-compositor/   # GPU compositor
│   ├── satya-shell/         # Desktop shell
│   ├── satya-theme/         # Theme packs
│   ├── lib-sea/             # UI component framework
│   └── boot-diagnostics/    # Boot overlay HUD
├── apps/
│   ├── display-manager/     # Login greeter
│   ├── file-manager/        # File browser
│   ├── terminal/            # Terminal emulator
│   ├── settings/            # System settings
│   ├── image-viewer/        # Image viewer
│   ├── bharat-browser/      # Web browser
│   ├── music/               # Music player
│   ├── video/               # Video player
│   ├── camera/              # Camera app
│   ├── calculator/          # Calculator
│   ├── calendar/            # Calendar
│   ├── notes/               # Notes
│   └── task-manager/        # Task manager
├── ai/
│   ├── bharat-ai/           # AI engine
│   ├── voice-daemon/        # Voice assistant
│   └── memory.rs            # Context memory
├── devtools/
│   ├── bharat-sdk/          # SDK
│   └── bharat-debugger/     # Debugger
└── docs/
    ├── ARCHITECTURE.md
    ├── KERNEL.md
    ├── FILESYSTEM.md
    ├── DESKTOP.md
    ├── AI.md
    └── ROADMAP.md
```

## Building

```bash
# Install Rust nightly for kernel development
rustup default nightly

# Build entire workspace
cargo build --workspace

# Build kernel only
cargo build -p bharat-kernel

# Build bootloader
cargo build -p bharat-boot

# Build apps
cargo build --workspace --apps
```

## Documentation

- [Architecture Overview](docs/ARCHITECTURE.md)
- [Kernel Design](docs/KERNEL.md)
- [Filesystem](docs/FILESYSTEM.md)
- [Desktop Environment](docs/DESKTOP.md)
- [AI Integration](docs/AI.md)
- [Security Model](docs/SECURITY.md)
- [API Reference](docs/API.md)
- [Roadmap](docs/ROADMAP.md)

## Development

See [AGENTS.md](AGENTS.md) for development agent guidelines.

## License

Apache-2.0

## Status

Active development — prototyping phase.
