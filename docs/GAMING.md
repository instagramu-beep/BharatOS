# BharatOS Gaming

## Graphics APIs

### Vulkan (Primary)
- Vulkan 1.3 fully supported
- Mesh shaders, ray tracing, variable rate shading
- Synchronization primitives
- Multi-GPU support

### OpenGL (Compatibility)
- OpenGL 4.6 Core
- OpenGL ES 3.2
- Compatibility profile for legacy games

### DirectX Compatibility Layer (VKCL)
- DX11 → Vulkan translation layer
- DX10/9 -> DX11 -> Vulkan
- Wine/Proton compatible

## Features

- **High Refresh Rates**: 120/144/240/360 Hz supported
- **HDR Gaming**: HDR10, Dolby Vision
- **Adaptive Sync**: VRR (G-Sync Compatible, FreeSync)
- **Game Mode**: Priority boost, background suspension, latency reduction
- **Low-latency audio pipeline**
- **Controller Support**: XInput, DirectInput, Steam Input, HID

## Optimizations

- **CPU Scheduler**: Real-time priority for game threads
- **GPU Scheduling**: Fine-grained command buffer submission
- **Memory**: Dedicated VRAM carve-outs, zero-copy
- **Display**: Exclusive full-screen, flip model
- **Input**: Low-latency input stack (1ms target)
- **Audio**: WASAPI-style exclusive mode, 128-sample buffer

## Anti-Cheat

- Kernel-mode anti-cheat compatible
- Secure input path
- Screen capture protection

## Recording & Streaming

- Built-in game capture (Gaming Overlay)
- Hardware-accelerated encoding
- Twitch/YouTube integration

## Store

- BharatAppStore game section
- Proton-based Windows game runner
- Native Vulkan game support
