# BharatOS Kernel Design

## Boot Sequence

1. **UEFI Firmware** loads `kernel.elf` via bootloader
2. **Bootloader** (bharat-boot):
   - Sets up GOP framebuffer
   - Queries memory map (UEFI GetMemoryMap)
   - Detects ACPI tables
   - Verifies Secure Boot signature
   - Loads kernel ELF into memory
   - Sets up BootInfo struct
   - Calls `ExitBootServices`
   - Jumps to kernel entry point
3. **Kernel Entry** (`_bharat_kernel_entry`):
   - Early HAL: disable PIC, init IDT, enable protected features
   - Physical memory manager initialization
   - Virtual memory setup (paging)
   - ACPI + CPU enumeration + SMP bring-up
   - APIC, local and I/O, interrupt routing
   - High-res timer, TSC calibration
   - Scheduler initialization
   - VFS mount (BharatFS root, procfs, devfs, sysfs)
   - Device discovery (PCIe, USB, GPU, storage, network)
   - Early services spawn
   - Desktop subsystem init
   - User-space launch (bharat-init)

## Subsystem Architecture

### Hardware Abstraction Layer (libhal)

- **MSR**: Model Specific Registers (EFER, PAT, FS/GS base, APIC base)
- **PIC**: Legacy 8259 controller (disabled on APIC systems)
- **APIC**: Local APIC, I/O APIC, MSI, MSI-X
- **HPET**: High Precision Event Timer (primary clock)
- **PIT**: Programmable Interval Timer (fallback/calibration)
- **PS/2**: Keyboard and mouse controller
- **ACPI**: Power management, device enumeration, sleep states
- **USB**: OHCI/UHCI/EHCI/xHCI host controllers
- **PCIe**: Device enumeration, capability parsing
- **GPU**: Display detection, framebuffer, GPU memory
- **Storage**: Block device enumeration, DMA setup
- **Network**: NIC detection, interrupt setup
- **Audio**: HDA codec detection
- **Input**: Keyboard, mouse, touch, pen enumeration
- **Power**: CPU frequency, display brightness, battery

### Memory Management (libmm)

- **Frame Allocator**: Buddy allocator, 4 KB frames
- **Kernel Heap**: Slab allocator for small objects, buddy for large
- **Paging**: 4-level page tables, 2 MB/1 GB huge pages
- **Page Cache**: File data caching, LRU eviction
- **Slab Cache**: Per-object-type caches (task, inode, etc.)

### Scheduler (libsched)

- **CFS Queue**: Red-black tree by vruntime
- **RT Queue**: 64 priority bands, FIFO + RR
- **Per-CPU**: One run queue per CPU (reduced contention)
- **Load Balancing**: Periodic migration, NUMA-aware
- **Affinity**: CPU set and NUMA node binding

### Virtual Filesystem (libfs)

- **VFS**: Abstract interface for all filesystems
- **BharatFS**: Custom journaling filesystem
- **procfs**: Process information (/proc)
- **devfs**: Device nodes (/dev)
- **tmpfs**: Temporary filesystem (/tmp)
- **Path resolution**: Component-by-component lookup with caching

### Security (libcore::security)

- **Sandbox**: seccomp-like filters, namespace isolation
- **Firewall**: Stateful packet inspection, connection tracking
- **Permissions**: Capability-based access control
- **Verified Boot**: Ed25519 signature verification
- **TPM**: Trusted Platform Module integration
- **Audit**: Security event logging

### Networking (libnet)

- **IP Stack**: IPv4, IPv6, routing, fragmentation
- **Transport**: TCP, UDP, SCTP
- **Security**: TLS 1.3, IPsec-offload capable
- **Socket API**: Berkeley sockets compatible

## Kernel ABI

### System Calls

| syscall | Arguments | Description |
|---------|-----------|-------------|
| `read` | fd, buf, len | Read from file descriptor |
| `write` | fd, buf, len | Write to file descriptor |
| `open` | path, flags, mode | Open file |
| `close` | fd | Close file descriptor |
| `mmap` | addr, len, prot, flags, fd, off | Map memory |
| `munmap` | addr, len | Unmap memory |
| `fork` | - | Create process |
| `exec` | path, argv, envp | Execute program |
| `exit` | status | Exit process |
| `wait` | pid, status, options | Wait for child |
| `kill` | pid, sig | Send signal |
| `pipe` | fds | Create pipe |
| `dup` | fd, newfd | Duplicate file descriptor |
| `ioctl` | fd, cmd, arg | Device control |
| `socket` | domain, type, protocol | Create socket |
| `bind` | sockfd, addr, addrlen | Bind socket |
| `listen` | sockfd, backlog | Listen for connections |
| `accept` | sockfd, addr, addrlen | Accept connection |

### BootInfo Structure

```rust
struct BootInfo {
    magic: u64,           // "BHARAT_OS" magic
    version: u32,
    session_id: u64,
    memory_map: u64,      // Physical address of memory map
    memory_map_size: u32,
    descriptor_size: u32,
    fps: u32,            // Framebuffer width
    fh: u32,             // Framebuffer height
    fpb: u32,            // Framebuffer pitch
    framebuffer: u64,    // Framebuffer physical address
    acpi_root: u64,      // ACPI RSDT/XSDT physical address
    cpu_count: u32,
    cpu_features: u64,
    secure_boot_enabled: u32,
}
```

## Development Guidelines

1. No unsafe code in library code (kernel space is exception)
2. All kernel code must be `#![no_std]`
3. Lock discipline: never hold a lock across blocking operations
4. All errors must be handled with proper recovery
5. No dynamic allocation in interrupt context
6. Code must compile with `--no-default-features` for kernel builds
