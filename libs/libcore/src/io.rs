//! IO traits for abstracting block devices, serial, framebuffer, and sockets
#![no_std]

pub mod driver {
    // use crate::time::Duration;
    
    /// Block device — read/write slices at block offsets
    pub trait BlockDevice: Sync {
        fn read_block(&self, block_no: u64, buf: &mut [u8]) -> crate::Result<()>;
        fn write_block(&mut self, block_no: u64, buf: &[u8]) -> crate::Result<()>;
        fn block_size(&self) -> u64 { 512 }
        fn count(&self) -> u64 { 0 }
        fn flush(&mut self) -> crate::Result<()> { Ok(()) }
        fn reset(&mut self) -> crate::Result<()> { Ok(()) }
    }

    /// Character device — streaming byte I/O (serial, USB)
    pub trait CharDevice: Sync {
        fn read(&self, buf: &mut [u8]) -> crate::Result<usize> { Ok(0) }
        fn write(&mut self, buf: &[u8]) -> crate::Result<usize>;
        fn available(&self) -> usize { 0 }
        fn poll(&self, mask: PollEvent) -> PollResult { PollResult::empty() }
    }

    /// Network device
    pub trait NetDevice: Sync {
        fn mac_addr(&self) -> [u8; 6];
        fn mtu(&self) -> u16;
        fn rx(&self, buf: &mut [u8]) -> crate::Result<usize>;
        fn tx(&mut self, buf: &[u8]) -> crate::Result<()>;
        fn link_up(&self) -> bool;
    }

    /// Input device (keyboard, mouse, touch, pen, controller)
    pub trait InputDevice: Sync {
        fn poll(&self) -> Option<InputEvent>;
        fn device_type(&self) -> InputDeviceType;
    }

    /// Audio device
    pub trait AudioDevice: Sync {
        fn sample_rate_supported(&self, rate: u32) -> bool;
        fn enqueue_samples(&mut self, samples: &[f32]) -> crate::Result<()>;
        // fn current_latency(&self) -> Duration;
        fn set_volume(&mut self, vol: f32);
    }

    /// GPU/VGA device
    pub trait GpuDevice: Sync {
        fn framebuffer(&self) -> Option<GpuFramebuffer>;
        fn modes(&self) -> &[ModeInfo];
        fn set_mode(&mut self, mode: u32) -> crate::Result<()>;
        fn blit(&mut self, fb: &GpuFramebuffer) -> crate::Result<()>;
    }

    /// Numeric types
    #[derive(Copy, Clone)]
    pub struct ModeInfo {
        pub width: u32,
        pub height: u32,
        pub bpp: u8,
        pub pitch: u32,
        pub base_addr: u64,
        pub double_buffered: bool,
    }
    pub struct GpuFramebuffer {
        pub pixels: &'static mut [u32],
        pub width: u32,
        pub height: u32,
        pub pitch: u32,
    }
    
    /// Poll event flags
    use bitflags::bitflags;
    bitflags! { pub struct PollEvent: u8 { const READ = 0b001; const WRITE = 0b010; const ERR = 0b100; } }
    bitflags! { pub struct PollResult: u8 { const READ = 0b001; const WRITE = 0b010; const ERR = 0b100; const HUP = 0b1000; } }
    
    #[derive(Copy, Clone)]
    pub enum InputDeviceType {
        Unknown, Keyboard, Mouse, TouchPanel, Pen, Gamepad, Tablet,
    }
    
    #[derive(Copy, Clone, Debug)]
    pub enum InputEvent {
        KeyDown(Keycode),
        KeyUp(Keycode),
        MouseMove { x: f64, y: f64 },
        MouseButton { btn: MouseButton, pressed: bool },
        Scroll { dx: f64, dy: f64 },
        TouchBegin(u32, f64, f64),
        TouchMove(u32, f64, f64),
        TouchEnd(u32),
    }
    
    #[derive(Copy, Clone, Debug)]
    pub enum Keycode {
        Unknown, A, B, C, D, E, F, G, H, I, J, K, L, M,
        N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
        Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
        Return, Escape, Backspace, Tab, Space, CtrlL, CtrlR, AltL, AltR, ShiftL, ShiftR,
        Up, Down, Left, Right,
        F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    }
    #[derive(Copy, Clone, Debug)]
    pub enum MouseButton { Left, Middle, Right, Back, Forward }
}
