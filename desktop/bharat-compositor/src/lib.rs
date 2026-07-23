//! BharatOS Graphics Compositor — GPU-accelerated 3D compositor with
//! Vulkan + Wayland windows, real-time lighting, animated wallpapers, particle
//! systems, glass/transparency, reflection, and 240 FPS animation target.
//!
//! The compositor runs as a highly privileged userspace process backed by the
//! kernel DMA-BUF infrastructure.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use libsurface::*;

pub mod pipeline;
pub mod renderer;
pub mod scene_graph;
pub mod shaders;
pub mod wallpaper;
pub mod texture_manager;
pub mod swapchain;
pub mod vsync;
pub mod effects;
pub mod raytracing;
pub mod frame_scheduler;
pub mod input_handler;

/// Minimal Vulkan-like renderer for BharatOS
pub struct BharatCompositor {
    pub renderer: renderer::GpuRenderer,
    pub scene: scene_graph::Scene,
    pub swap: swapchain::Swapchain,
    pub wallpaper: wallpaper::WallpaperEngine,
    pub effects: effects::EffectStack,
    pub input: input_handler::InputRouter,
    pub config: CompositorConfig,
}

#[repr(C)]
pub struct CompositorConfig {
    pub target_fps: u32,
    pub triple_buffered: bool,
    pub vsync_enabled: bool,
    pub hdr_enabled: bool,
    pub bloom_intensity: f32,
    pub blur_kernel_radius: f32,
    pub reflection_amount: f32,
    pub lighting_model: LightingModel,
    pub wallpaper_id: u32,
    pub particle_count: u32,
    pub gpu_node: u32,
    pub dpm: DisplayPowerManagement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightingModel {
    Default,
    HDR = 1,
    PathTracer = 2,
    AmbientOcclusion = 3,
    Normal = 4,
    Impression = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayPowerManagement {
    pub timeout_secs: u32,
    pub dim_factor: f32,        // 0-1
    pub turn_off_on_idle: bool,
    pub night_dim: bool,
    pub adaptive_brightness: bool,
}

impl Default for CompositorConfig {
    fn default() -> Self {
        Self {
            target_fps: 240,
            triple_buffered: true,
            vsync_enabled: true,
            hdr_enabled: true,
            bloom_intensity: 0.15,
            blur_kernel_radius: 3.24,
            reflection_amount: 0.80,
            lighting_model: LightingModel::HDR,
            wallpaper_id: 0,
            particle_count: 4096,
            gpu_node: 0,
            dpm: DisplayPowerManagement {
                timeout_secs: 60,
                dim_factor: 0.30,
                turn_off_on_idle: true,
                night_dim: true,
                adaptive_brightness: true,
            },
        }
    }
}