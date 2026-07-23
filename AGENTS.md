# BharatOS Development Agents Guide

## Development Workflow

### 1. Kernel Changes

- All kernel code must compile with `#![no_std]` and `#![allow(unused)]`
- Use `kernel_log!` macros, never `println!`
- Memory allocated in interrupt context must come from pre-allocated pools
- All locks must be documented: what they protect, expected hold time

### 2. Driver Development

- New drivers go in `libs/libhal/src/<bus>.rs`
- Follow the template: probe → init → register → interrupt handler
- All hardware access must be through safe abstractions
- Register with the device manager

### 3. Filesystem Development

- New filesystems implement `VfsFilesystem`
- Register via `libfs::register_filesystem(name, constructor)`
- Use BharatFS utilities for formatting/checking

### 4. Application Development

- Apps use `libsurface` for rendering
- Communicate with system via IPC (sockets, messages)
- Follow Material-inspired design guideline
- All strings must be UTF-8

### 5. AI Feature Development

- Models go in `/var/lib/ai/models/`
- Use `libaep` for inference
- Respect privacy policy: no external data without consent
- All AI features must have offline fallback

### 6. Security Review

- All code touching security-sensitive paths needs review
- Use `security::audit()` for security event logging
- Follow capability-based model for permissions

## Code Style

- 4 spaces, no tabs
- `snake_case` for functions/variables
- `PascalCase` for types
- `SCREAMING_SNAKE_CASE` for constants
- No trailing whitespace
- Maximum line length: 100 characters
- Document public API with doc comments

## Commit Messages

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: feat, fix, docs, style, refactor, perf, test, build, ci, revert, chore

## Testing

- Unit tests in `#[cfg(test)]` blocks
- Kernel tests in `kernel/tests/`
- Integration tests in `tests/`
- Use `cargo test` for all tests

## Review Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] No new unsafe code without review
- [ ] Documentation updated
- [ ] CHANGELOG updated
