//! BharatOS Security Framework — sandbox, firewall, permissions, verified updates
#![no_std]
#![allow(unused)]

use crate::prelude::*;

pub mod sandbox;
pub mod firewall;
pub mod permissions;
pub mod capabilities;
pub mod verified_boot;
pub mod trusted_platform;
pub mod audit;
pub mod intrusion_detection;
pub mod secure_storage;

pub use sandbox::*;
pub use firewall::*;
pub use permissions::*;
pub use capabilities::*;

bitflags::bitflags! {
    pub struct SecurityFlags: u64 {
        const SECURE_BOOT = 1 << 0;
        const VERIFIED_UPDATES = 1 << 1;
        const SANDBOX_APPS = 1 << 2;
        const MANDATORY_ACCESS_CONTROL = 1 << 3;
        const FIREWALL_ENFORCED = 1 << 4;
        const ENCRYPTED_STORAGE = 1 << 5;
        const SECURE_NETWORKING = 1 << 6;
        const BIOMETRIC_AUTH = 1 << 7;
        const TPM_ATTESTATION = 1 << 8;
        const ZERO_TRUST = 1 << 9;
        const PRIVACY_MODE = 1 << 10;
        const ANTI_MALWARE = 1 << 11;
    }
}

pub struct SecurityManager {
    pub flags: SecurityFlags,
    pub firewall_rules: [FirewallRule; 256],
    pub rule_count: usize,
    pub sandbox_policies: BTreeMap<AppId, SandboxPolicy>,
    pub audit_log: [AuditEntry; 1024],
    pub audit_idx: usize,
    pub tpm: Option<&'static mut dyn TrustedPlatformModule>,
    pub keyring: Keyring,
}

impl SecurityManager {
    pub const fn new() -> Self {
        Self {
            flags: SecurityFlags::all(),
            firewall_rules: unsafe { core::mem::zeroed() },
            rule_count: 0,
            sandbox_policies: BTreeMap::new(),
            audit_log: unsafe { core::mem::zeroed() },
            audit_idx: 0,
            tpm: None,
            keyring: Keyring::new(),
        }
    }

    pub fn init() -> &'static mut Self {
        unsafe { &mut GLOBAL_SECURITY }
    }

    pub fn add_rule(&mut self, rule: FirewallRule) {
        if self.rule_count < 256 {
            self.firewall_rules[self.rule_count] = rule;
            self.rule_count += 1;
            self.audit(SecurityEvent::FirewallRuleAdded { rule_idx: self.rule_count as u32 - 1 });
        }
    }

    pub fn audit(&mut self, event: SecurityEvent) {
        let entry = AuditEntry {
            timestamp: timestamp(),
            event,
            pid: process::current_pid(),
            uid: process::current_uid(),
            result: AuditResult::Success,
        };
        let idx = self.audit_idx % 1024;
        self.audit_log[idx] = entry;
        self.audit_idx += 1;
    }

    pub fn enforce_app_sandbox(&self, app: AppId) -> SandboxPolicy {
        self.sandbox_policies.get(&app).cloned().unwrap_or_default()
    }

    pub fn verify_file_integrity(&mut self, path: &str, hash: &[u8; 32]) -> bool {
        let actual_hash = self.hash_file(path);
        actual_hash == *hash
    }
}

static mut GLOBAL_SECURITY: SecurityManager = SecurityManager::new();
