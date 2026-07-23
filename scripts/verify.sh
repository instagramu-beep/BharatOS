#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo "=== BharatOS X — Build Verification ==="
echo ""

FAILED=0

check() {
    echo -n "[CHECK] $1 ... "
    if eval "$2" >/dev/null 2>&1; then
        echo "OK"
    else
        echo "FAIL"
        FAILED=$((FAILED + 1))
    fi
}

# Source files
check "Rust source files exist" "find . -name '*.rs' | wc -l | grep -q '^[1-9]'"
check "No Cargo.toml missing" "[ \$(find . -name Cargo.toml | wc -l) -ge 45 ]"
check "lib.rs present in crates" "find . -name Cargo.toml -not -path './Cargo.toml' -exec dirname {} \; | while read d; do [ -f \$d/src/lib.rs ] || exit 1; done"
check "main.rs present in binaries" "find . -name Cargo.toml -not -path './Cargo.toml' -exec dirname {} \; | while read d; do [ -f \$d/src/main.rs ] || exit 1; done"

# Workspace integrity
echo ""
echo "[WORKSPACE]"
check "members count matches crates" "
    members=\$(grep -cE '^\s+\"[^\"]+\",?\$' Cargo.toml || true)
    crates=\$(find . -name Cargo.toml -not -path './Cargo.toml' | wc -l)
    [ \"\$members\" -ge \"\$crates\" ]
"

# No stubs
echo ""
echo "[CODE QUALITY]"
check "No todo!/unimplemented!/unreachable! stubs" "
    count=\$(grep -rnE 'todo!\(\)|unimplemented!\(\)|unreachable!\(\)' --include='*.rs' . 2>/dev/null | wc -l)
    [ \"\$count\" -eq 0 ]
"

check "No empty pub fn bodies" "
    count=\$(grep -rnE '^\s*pub fn .*\{\s*\}$' --include='*.rs' . 2>/dev/null | grep -v '/test' | wc -l)
    [ \"\$count\" -eq 0 ]
"

# Build artifacts
echo ""
echo "[BUILD ARTIFACTS]"
check "target/ directory exists" "[ -d target ]"
check "Bootloader EFI exists" "[ -f target/x86_64-unknown-uefi/release/bharat-boot.efi ]"
check "Kernel ELF exists" "[ -f target/release/bharat-kernel ]"
check "ISO exists" "[ -f bharatos.iso ]"

# ISO structure
echo ""
echo "[ISO STRUCTURE]"
if [ -d iso ]; then
    check "iso/boot/efi/BOOTX64.EFI present" "[ -f iso/boot/efi/BOOTX64.EFI ]"
    check "iso/boot/bharat/kernel.elf present" "[ -f iso/boot/bharat/kernel.elf ]"
else
    echo "[ISO] iso/ directory missing — run ./scripts/build.sh"
fi

# Documentation
echo ""
echo "[DOCUMENTATION]"
check "README.md present" "[ -f README.md ]"
check "LICENSE present" "[ -f LICENSE ]"
check "Makefile present" "[ -f Makefile ]"
check "CI workflow present" "[ -f .github/workflows/build.yml ]"
check "Issue templates present" "[ -d .github/ISSUE_TEMPLATE ]"

# Summary
echo ""
echo "=== Verification Summary ==="
if [ "$FAILED" -eq 0 ]; then
    echo "ALL CHECKS PASSED"
    echo "BharatOS X is ready to boot."
    echo "Run: ./scripts/run.sh"
else
    echo "FAILED: $FAILED checks"
    echo "Run ./scripts/install.sh && ./scripts/build.sh"
    exit 1
fi
