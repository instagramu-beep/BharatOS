//! BharatOS Boot Diagnostics overlay — visible over SMP boot phase
//!
//! BootDiagnostics renders as a transparent "HUD" window over the 3D wallpaper
//! while processes are starting. Panels for:
//!   - Kernel boot phase progress bar (12 stages)
//!   - CPU initialization graph (1 bar per core)
//!   - Memory configuration (DRAM channel count, capacity, clock)
//!   - GPU bring-up (GPU model, VRAM, clocks)
//!   - Storage enumeration (SATA / NVMe / USB names + speeds)
//!   - Network bring-up (link status, IP, link speed)
//!   - Boot timer timestamps (each phase's delta in ms)
//!   - FPS panel (compostior) showing FPS + frame time
//!
//! The overlay fades out automatically when the compositor produces the user's
//! first frame (PHASE_FULLY_RUNNING) and the user can dismiss it via a hotkey.

use crate::prelude::*;

pub struct BootDiagnosticsLayer {
    pub enabled: bool,
    pub overlay: GraphicsOverlay,
    pub panels: [HeatmapState; 8],
    pub fps_history: [f32; 120],
    pub clock_max: f32,
    pub cpu_history: [f32; 256],
    pub memory_channel_count: u8,
}

#[derive(Clone, Copy)]
pub struct HeatmapState {
    pub label: [u8; 32],
    pub value: f32,            // 0-100 score
    pub color: u32,            // rgba8888
    pub history: [f32; 60],    // last 60 frames
}

pub struct GraphicsOverlay {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub opacity: f32,
    pub z_index: u32,
    pub anim_dur_ms: u32,
}

impl BootDiagnosticsLayer {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }
    
    pub fn begin(&mut self) {
        self.enabled = true;
    }
    
    pub fn update(&mut self, phase: &BootPhaseInfo) {
        for (panel, info) in self.panels.iter_mut().zip(phase.channels.iter()) {
            panel.value = info.percent_complete();
            panel.history.rotate_left(1);
            panel.history[59] = panel.value;
        }
        self.overlay.opacity = (phase.global_progress() as f32 / 100.0) * 0.95;
    }
    
    pub fn render(&mut self, ctx: &mut PaintContext) {
        if !self.enabled { return; }
        
        // Draw semi-transparent background
        ctx.fill_rect(self.overlay.x, self.overlay.y, self.overlay.w, self.overlay.h, 
                      RGBA8(0x10, 0x10, 0x20, (self.overlay.opacity * 255.0) as u8));
        
        // Draw panels
        for (i, panel) in self.panels.iter().enumerate() {
            let py = self.overlay.y + i as u32 * 40;
            
            // Progress bar background
            ctx.fill_rect(self.overlay.x, py, self.overlay.w, 24, RGBA8(0x2A, 0x2A, 0x3A, 0xC0));
            
            // Progress bar fill
            let fill_w = (self.overlay.w as f32 * panel.value / 100.0) as u32;
            ctx.fill_rect(self.overlay.x, py, fill_w, 24, panel.color);
            
            // Label text
            let label = unsafe { 
                core::str::from_utf8_unchecked(&panel.label)
            };
            ctx.draw_text(self.overlay.x + 10, py + 6, label, 14, TEXT_COLOR);
        }
        
        // FPS counter (bottom right)
        let current_fps = self.fps_history[119];
        ctx.draw_text(self.x + w - 80, self.y + h - 36, 
                      &format!("{:.0} FPS", current_fps), 14, TEXT_COLOR);
    }
}

#[derive(Clone, Copy)]
pub struct BootPhaseInfo {
    pub phases: [f32; 12],          // 0-100%
    pub boot_session_id: u64,
    pub total_bytes: u64,
    pub staged_bytes: u64,
    pub devices_enumerated: u32,
}

impl BootPhaseInfo {
    pub fn global_progress(&self) -> u8 {
        let sum: f32 = self.phases.iter().sum();
        (sum / 12.0) as u8
    }
}

#[repr(u32)]
pub enum BootPhase {
    HALInit = 0,
    MemoryInit = 1,
    AcpiEnum = 2,
    IntrSetup = 3,
    TimerInit = 4,
    SchedInit = 5,
    VfsMount = 6,
    DeviceEnum = 7,
    ServiceSpawn = 8,
    DesktopInit = 9,
    UserspaceInit = 10,
    FullyRunning = 11,
}

#[inline(always)]
pub fn on_phase(phase: BootPhase) {
    // Notify diagnostics layer
}
const RGBA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r | (g << 8) | (b << 16) | (a << 24))
}