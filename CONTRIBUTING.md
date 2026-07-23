# Contributing to BharatOS

Thank you for your interest in contributing to BharatOS!

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to
uphold this code. Report unacceptable behavior to conduct@bharatos.dev.

## How to Contribute

### Reporting Bugs

- Use the bug report template
- Include OS version, hardware details, and reproduction steps
- Attach relevant logs from `/var/log/bharat/`

### Suggesting Features

- Use the feature request template
- Explain the use case and expected behavior
- Consider whether it fits the project scope

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes following the coding style in AGENTS.md
4. Run `make check` and `make test`
5. Commit with a clear message following the convention:
   ```
   feat(scope): short description

   Longer explanation if needed.
   ```
6. Push to your fork and open a PR

### Coding Standards

- 4 spaces, no tabs
- `snake_case` for functions/variables
- `PascalCase` for types
- `SCREAMING_SNAKE_CASE` for constants
- Maximum line length: 100 characters
- Document public API with doc comments

### Kernel Code Rules

- All kernel code must be `#![no_std]`
- Use `kernel_log!` macros, never `println!`
- No dynamic allocation in interrupt context
- All locks must document what they protect and expected hold time

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `revert`, `chore`

## Development Setup

```bash
# Install Rust nightly
rustup default nightly

# Clone the repository
git clone https://github.com/bharatos/bharatos.git
cd bharatos

# Build
make build

# Run tests
make test

# Run in QEMU
make qemu
```

## Review Process

- All PRs require at least one approval from a maintainer
- Kernel changes require review from at least two maintainers
- CI must pass before merge
- Maintainers will review within 5 business days

## Recognition

Contributors will be recognized in the release notes and on the project website.
