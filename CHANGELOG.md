# Changelog

All notable changes to BharatOS X will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-01

### Added
- Custom BharatOS kernel with preemptive multitasking
- UEFI bootloader with Secure Boot support (x86_64, ARM64, RISC-V)
- BharatFS journaling filesystem with COW, snapshots, compression, encryption
- CFS + RT hybrid scheduler with per-CPU run queues
- Physical memory manager (buddy allocator) + kernel heap
- Virtual filesystem layer with procfs, devfs, tmpfs
- HAL layer: MSR, PIC, APIC, IDT, serial, timer, PCIe, USB, GPU, storage, ACPI
- Network stack foundation
- Security framework: sandbox, firewall, capabilities, audit logging
- IPC: pipes, message queues, shared memory
- Satya Desktop environment with 3D compositor
- 18 first-party applications
- BharatAI daemon with offline model support
- Voice daemon with STT/TTS
- Package manager (BharatPkg) with delta updates
- SDK and debugger

### Documentation
- Architecture, kernel, filesystem, desktop, AI, security, hardware, gaming, compiler, power, translator, API, roadmap

[0.2.0]: https://github.com/bharatos/bharatos/releases/tag/v0.2.0
[0.1.0]: https://github.com/bharatos/bharatos/releases/tag/v0.1.0
