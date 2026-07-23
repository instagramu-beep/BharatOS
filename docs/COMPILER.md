# BharatOS Compiler Toolchain

## BharatCC Compiler

BharatOS ships with a native compiler toolchain:

### Components

- **bhcc** — Bharat C Compiler (LLVM-based, BharatOS target)
- **bhc++** — Bharat C++ Compiler
- **bhld** — Bharat Linker (bitcode linker + native linker)
- **bharat-sysroot** — Standard library and headers

### Language Support

- C11, C17, C2x
- C++17, C++20, C++23
- BharatScript (native scripting)
- Rust (via rustc, first-class)
- Assembly (AT&T and Intel syntax)

### Target Triples

- `BHARAT_X86_64` — x86-64 (primary)
- `BHARAT_AARCH64` — ARM64
- `BHARAT_RISCV64` — RISC-V 64

### Build System

```bash
bhcc -target BHARAT_X86_64 -O2 -o app.o app.c
bhld app.o -lbharat -o app
```

### Optimization

- LTO (Link-time optimization)
- PGO (Profile-guided optimization)
- Auto-vectorization (SSE/AVX2/NEON)
- Loop unrolling and fission
- Interprocedural analysis

### Installation

```bash
# Install SDK
sathya-pkg install bharat-sdk

# Verify installation
bhcc --version
```
