//! BharatOS Settings — centrally configured user preferences, security policy, UEFI
//! boot-order, network policy, battery / energy (BHMS), physec (BHMS) / etc.
//!
//! All immutable user preferences are stored in the user's BharatFS home, at
//! $HOME/.config/baratos/defaults.toml. The settings app re-maps to per-user scope
//! with Protected-USB-Media credentials.

#![no_std]

use libsurface::prelude::*;
use crate::prelude::*;
use libcore::fs::toml;

pub mod model;
pub mod panes;
pub mod firmware;
pub mod battery_pane;
pub mod graphics_pane;
pub mod network_pane;
pub mod security_pane;
pub mod user_pane;
pub mod input_pane;
pub mod sound_pane;
pub mod display_pane;
pub mod accessibility_pane;

bitflags::bitflags! {
    pub struct SecurityFlags: u64 {
        const ENFORCE_SECURE_BOOT    = 1 << 0;
        const VERIFIED_SIGNED_KERNEL = 1 << 1;
        const ENFORCE_KEXEC_KERNEL   = 1 << 2;
        const ENFORCE_KEXEC_INITRD   = 1 << 3;
        const LOCK_DOWN_FIRMWARE     = 1 << 4;
        const DISABLE_RT             = 1 << 5;
        const FORCE_DEVPT            = 1 << 6;
        const SANDBOX_APPS           = 1 << 7;
        const ALLOW_UNSIGNED_APPS    = 1 << 8;
        const ENFORCE_CANONICAL      = 1 << 9;
        const DISABLE_KPAR           = 1 << 10;
        const ENFORCE_SPINLOCK       = 1 << 11;
        const GATE_RDTSC             = 1 << 12;
        const FORCE_TPM_ATTEST       = 1 << 13;
        const TVM_SANDBOX            = 1 << 14;
    }
}

#[derive(Clone, Debug)]
pub struct SettingsModel {
    pub general: GeneralSettings,
    pub security: SecuritySettings,
    pub display: DisplaySettings,
    pub audio: AudioSettings,
    pub input: InputSettings,
    pub battery: BatterySettings,
    pub network: NetworkSettings,
    pub bluetooth: BluetoothSettings,
    pub firmware: FirmwareSettings,
    pub notifications: NotificationSettings,
    pub desktop: DesktopSettings,
    pub accessibility: AccessibilitySettings,
    pub developer: DeveloperSettings,
    pub cloud: CloudSettings,
    pub updates: UpdatesSettings,
}

#[derive(Clone, Debug)]
pub struct SecuritySettings {
    pub flags: SecurityFlags,
    pub repudiation_set: RepudiationSet,
    pub attestation_mode: AttestationLevel,
    pub sandbox_seals: BTreeSet<Fingerprint>,
    pub mandatory_trust_layer: MandatoryPartitionTrust,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttestationLevel {
    TU = 0,
    DTU = 1,
    TPU = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct RepudiationSet {
    pub repudiation_mode: RepudiationMode,
    pub archive_on_deny: bool,
}

#[repr(u8)]
pub enum RepudiationMode { Deny, Allow, Notify, BlackList }

#[repr(u8)]
pub enum MandatoryPartitionTrust { Always, WhenAltered, Never, Conditional }

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            flags: SecurityFlags::all(),
            repudiation_set: RepudiationSet { repudiation_mode: RepudiationMode::Deny, archive_on_deny: true },
            attestation_mode: AttestationLevel::TPU,
            sandbox_seals: BTreeSet::new(),
            mandatory_trust_layer: MandatoryPartitionTrust::Always,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FirmwareSettings {
    pub secure_boot: bool,
    pub signed_osloader: bool,
    pub signed_kernel: bool,
    pub kernel_cmdline: [u8; 256],
    pub smm_lock: bool,
    pub cpu_lock: bool,
    pub bios_interface: BiosInterface,
    pub boot_order: Vec<BootEntry>,
    pub tpm_owner: Option<Fingerprint>,
}

#[derive(Clone, Debug)]
pub struct BootEntry {
    pub label: [u8; 64],
    pub efi_path: [u8; 64],
    pub efi_file_path_bytes: u64,
    pub kind: BootKind,
}

#[repr(u8)]
pub enum BootKind { BharatOS, Linux, Windows, MacOS, GenericUEFI, ChainloadPXE, NetworkISO }
pub type Fingerprint = [u8; 16];

pub struct SettingsWindow(surface::WindowHandle);

impl SettingsWindow {
    pub fn new() -> Self {
        let ctrl: Box<SettingsControl>;
        Self {
            surface: Surface::new_window("BharatOS Settings\0"),
            controller: SettingsController::from_model(SettingsModel::default()),
            action_tx: CrossbeamCross::unbounded(),
        }
    }
    
    pub fn render_pane(&mut self, pane: SettingsPane, ctx: &mut PaintContext) {
        match pane.main_split() {
            SplitKind::Sidebar => {},
            _ => {}
        }
    }
    
    pub fn on_python_cli(&mut self) -> Result<Action> {
        let _ = self;
        Err(err::Error::NotSupported)
    }
}
