# Satya Desktop — BharatOS Desktop Environment

## Satya Desktop Shell (satya-shell)

Satya is a GPU-accelerated, 3D desktop environment built on BharatCompositor.

### Architecture

```
┌─────────────────────────────────────────────┐
│             Satya Desktop Shell             │
├─────────────────────────────────────────────┤
│  ├─ Window Manager (3D, composited)         │
│  ├─ Panel System (top, bottom, side)        │
│  ├─ Dock (smart, animated)                  │
│  ├─ App Launcher (grid/search/voice)        │
│  ├─ Virtual Desktop Manager                │
│  ├─ Notification Center                    │
│  ├─ System Tray                            │
│  └─ Settings Integration                   │
├─────────────────────────────────────────────┤
│          BharatCompositor (Vulkan/Wayland)   │
├─────────────────────────────────────────────┤
│          libsurface (GPU Surface API)         │
├─────────────────────────────────────────────┤
│          libcore (no_std core library)        │
└─────────────────────────────────────────────┘
```

### Features

- **240 FPS** animations throughout
- **Glass/Blur** window decorations
- **Dynamic Shadows** — shadow blur based on window depth
- **Reflections** — floor/desk reflections on 3D surfaces
- **Live 3D Wallpapers** — animated, interactive, weather-aware
- **Particle Effects** — window open/close transitions
- **Virtual Desktops** — swipe/gesture to switch
- **Gesture Navigation** — swipe up/down/left/right
- **AI Organization** — smart window placement, workspace suggestions
- **Voice Control** — launch apps, search, control
- **Accessibility** — screen reader, magnifier, high contrast

## BharatCompositor

The compositor is a Wayland-compatible, Vulkan-based 3D compositor.

### Rendering Pipeline

1. **Scene Graph** — hierarchical scene representation
2. **Render Pass** — Vulkan render passes per frame
3. **Post-processing** — bloom, tone mapping, FXAA
4. **Swapchain** — triple-buffered presentation

### Wallpaper Engine

- Supports static, animated, live 3D wallpapers
- Weather integration (dynamic clouds, rain, snow)
- Interactive desktop objects (clickable)
- Audio visualizer mode
- Companion mode (follows mouse)

### Effects Stack

- SSAO (Screen Space Ambient Occlusion)
- SS Reflections (planar reflections)
- Motion blur
- Depth of field
- Chromatic aberration (optional)
- Film grain

## Theme System

- Material-inspired design language
- Rounded corners, dynamic shadows
- Light/Dark/Auto theme switching
- Custom accent colors
- System-wide font scaling

## Widgets

- Clock/Calendar
- System Monitor (CPU, RAM, network)
- Weather
- Quick Settings (volume, brightness, network)
- AI Assistant widget
- App shortcuts

## Lib-Sea

Satya UI component framework:
- Cross-process rendering via surface handles
- GPU-accelerated widgets
- Reactive layout engine
- Animation timeline
