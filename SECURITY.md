# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | Yes                |
| 0.9.x   | Security fixes only|
| < 0.9   | No                 |

## Reporting a Vulnerability

Please report security vulnerabilities to security@bharatos.dev.

- Include detailed description and reproduction steps
- Allow up to 90 days for triage and response
- Do NOT open public GitHub issues for security vulnerabilities

## Security Features

 BharatOS includes:
- Secure Boot with Ed25519 verification
- Full-disk encryption (AES-256-GCM, ChaCha20-Poly1305)
- Kernel page table isolation (KPTI)
- Stack canaries and CFI
- Sandboxed applications with capability model
- Firewall and intrusion detection
- TPM 2.0 attestation
- Verified updates with rollback

## Disclosure Policy

We follow coordinated disclosure:
1. Report received and confirmed
2. Fix developed and tested
3. Security advisory published
4. Patch released
