# BharatOS X — Architecture Overview

## 1. Kernel Architecture

BharatOS uses a monolithic kernel with clean module boundaries. The kernel is written entirely in Rust (no C/C++) for memory safety.

### 1.1 Process Model

- **Process**: isolated virtual address space, resources, threads
- **Thread**: schedulable execution context within a process
- **Kernel threads**: run in kernel space, no user address space
- **Scheduler**: CFS (Completely Fair Scheduler) for normal tasks, dedicated RT queue for real-time tasks
- **Preemption**: voluntary yield on normal tasks, preemptive on RT and on timer interrupt

### 1.2 Memory Model

- 4-level paging on x86-64 (PML4 → PDP → PD → PT)
- Higher-half kernel mapped at `0xFFFFFFFF80000000`
- User space mapped at `0x0000000001000000` → `0x00007FFFFFFFFFFF`
- Physical memory manager: buddy allocator (2^0 to 2^20 pages)
- Kernel heap: slab allocator for small objects, buddy for large
- Page flags: present, writable, user, huge, global, no-exec, COW, encrypted, device

### 1.3 Interrupt Model

- APIC (Advanced PIC) for interrupt delivery
- 256 interrupt vectors
- Local APIC per CPU, I/O APIC per chipset
- MSI/MSI-X for PCIe devices
- IRQ remapping via interrupt remapping table (IRT)

## 2. Filesystem Architecture

BharatFS is a copy-on-write, journaling filesystem designed for modern storage.

### 2.1 On-Disk Layout

```
[ Superblock (1 block) ]
[ Inode Table ]
[ Journal Area ]
[ Data Blocks ]
[ Snapshot Table ]
[ Free Space Bitmaps ]
```

### 2.2 Key Features

- **COW**: all modifications write to new blocks
- **Journaling**: metadata operations journaled for crash safety
- **Snapshots**: cheap, instant, consistent
- **Compression**: LZ4 for metadata, ZSTD for data
- **Deduplication**: content-based (SHA-256)
- **Encryption**: per-file or per-volume (AES-256-GCM)
- **Self-healing**: checksum validation, automatic repair

### 2.3 VFS Layer

- Virtual filesystem abstracts all filesystems
- Mount points form a tree
- File descriptors are per-process handles
- Path resolution with caching

## 3. Security Architecture

### 3.1 Boot Security

- Secure Boot: kernel signed with Ed25519, verified by UEFI
- Measured Boot: TPM extends PCRs with boot component hashes
- Boot Guard: BIOS/firmware integrity verification

### 3.2 Kernel Security

- SMAP/SMEP enforced
- Kernel page table isolation (KPTI)
- Stack canaries on all kernel functions
- CFI (Control Flow Integrity)
- Strict module signing requirement

### 3.3 Application Security

- Capability-based security model
- Sandboxing via seccomp-bpf equivalent
- Mandatory Access Control (MAC)
- Permission manager with fine-grained control

### 3.4 Network Security

- Firewall with stateful inspection
- TLS 1.3 everywhere (no plaintext HTTP)
- DNSSEC validation
- Network namespace isolation

## 4. Graphics Architecture

### 4.1 Compositor

- Vulkan-based 3D compositor (BharatCompositor)
- Wayland protocol compatible
- HDR, VRR, adaptive sync support
- 240 FPS target with triple buffering

### 4.2 Rendering Pipeline

```
App → Surface → GPU Command Buffer → Swapchain → Framebuffer
```

### 4.3 Effects

- Real-time lighting (PBR)
- Glass/blur effects
- Dynamic shadows and reflections
- Particle systems
- Animated/live wallpapers

## 5. AI Architecture

### 5.1 BharatAI Daemon

- Central AI runtime with offline/online modes
- Pipeline-based request processing
- Model registry (GGUF, ONNX, custom format)
- Privacy-first: on-device inference by default

### 5.2 Voice

- Voice Daemon: STT/TTS pipeline
- Wake-word detection (always-on, low-power)
- Audio processing chain: AEC, NS, AGC, VAD

### 5.3 Context

- Conversation memory with session management
- User preference learning
- Pattern recognition for suggestions
- Cross-app context sharing

## 6. Network Stack

### 6.1 Architecture

- Zero-copy packet I/O
- Per-CPU receive queues
- BPF packet filtering
- Socket layer with cork/enqueue

### 6.2 Protocols

- IPv4, IPv6 with stateless address autoconfiguration
- TCP with BBR congestion control
- UDP with pacing
- TLS 1.3 (ring-based)
- QUIC (HTTP/3)

## 7. Power Management

### 7.1 CPU

- Frequency scaling (Intel Speed Shift, AMD CPPC)
- C-states (C1-C7)
- Package C-state coordination

### 7.2 Display

- DPMS (Display Power Management Signaling)
- Adaptive brightness
- HDR tone mapping

### 7.3 Battery

- Charge level monitoring
- Discharge rate estimation
- Power profile switching (Performance/Balanced/Battery/Eco)

## 8. Storage Architecture

### 8.1 Block Layer

- IO_URING-style submission queues
- Multi-queue block layer
- NVMe with multiple I/O submission queues
- SATA with NCQ

### 8.2 Caching

- Page cache for file data
- Metadata cache (dcache)
- Inode cache
- Buffer cache for block I/O

## 9. Driver Model

### 9.1 Architecture

- Userspace drivers (IOMMU-protected)
- Kernel drivers for critical paths
- Driver model: device → driver → bus
- Hot-plug and power management callbacks

### 9.2 Supported Buses

- PCI, PCIe, PCI-X
- USB 1.1/2.0/3.0/3.1/3.2
- SATA, NVMe
- SD/MMC
- Bluetooth, Wi-Fi

## 10. Package Management

### 10.1 BharatPkg

- Delta updates (bsdiff-based)
- Dependency resolution (SAT solver)
- Sandboxed installation
- Atomic rollback
- Cryptographically signed packages
- Repository mirror support

### 10.2 Package Format

- Compressed tarball with embedded metadata
- Manifest with file hashes, permissions, dependencies
- post-install hooks in sandboxed environment
