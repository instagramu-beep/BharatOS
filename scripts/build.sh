#!/usr/bin/env bash
set -euo pipefail

echo "=== BharatOS X — Build ISO ==="

# Ensure we're in the project root
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo "Project root: $PROJECT_ROOT"
echo "Build type:   release"
echo "Target:       x86_64-unknown-uefi (boot) + x86_64-unknown-none (kernel)"
echo ""

# Load cargo env if available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Verify tools
echo "[CHECK] Verifying build tools..."
for cmd in cargo rustc qemu-system-x86_64 xorriso; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "MISSING: $cmd"
        echo "Run: ./scripts/install.sh"
        exit 1
    fi
done
echo "  All tools present."
echo ""

# Verify workspace integrity
echo "[CHECK] Verifying workspace members..."
MEMBER_COUNT=$(grep -cE '^\s+"[^"]+",?$' Cargo.toml || true)
CRATE_COUNT=$(find . -name Cargo.toml -not -path './Cargo.toml' | wc -l)
if [ "$MEMBER_COUNT" -ne "$((CRATE_COUNT))" ]; then
    echo "WARNING: Workspace members ($MEMBER_COUNT) != actual crates ($CRATE_COUNT)"
fi
echo "  Members: $MEMBER_COUNT, Crates: $CRATE_COUNT"
echo ""

# Clean previous builds
echo "[CLEAN] Removing previous build artifacts..."
rm -rf iso/ target/ bharatos.iso bharatos.img
echo "  Clean."
echo ""

# Build workspace
echo "[BUILD] Building BharatOS workspace..."
echo "  Pass 1: cargo check (host)"
if ! cargo check --workspace 2>&1 | tail -5; then
    echo "ERROR: cargo check failed"
    exit 1
fi
echo "  Pass 1 complete."
echo ""

echo "  Pass 2: bootloader (UEFI target)"
mkdir -p iso/boot/efi iso/boot/bharat
if ! cargo build --release -p bharat-boot --target x86_64-unknown-uefi 2>&1 | tail -5; then
    echo "ERROR: bootloader build failed"
    exit 1
fi
echo "  Pass 2 complete."
echo ""

echo "  Pass 3: kernel"
if ! cargo build --release -p bharat-kernel 2>&1 | tail -5; then
    echo "ERROR: kernel build failed"
    exit 1
fi
echo "  Pass 3 complete."
echo ""

# Locate artifacts
echo "[ARTIFACTS] Locating built binaries..."
BOOT_EFI="target/x86_64-unknown-uefi/release/bharat-boot.efi"
KERNEL_ELF="target/release/bharat-kernel"

if [ ! -f "$BOOT_EFI" ]; then
    echo "ERROR: Missing $BOOT_EFI"
    exit 1
fi
if [ ! -f "$KERNEL_ELF" ]; then
    echo "ERROR: Missing $KERNEL_ELF"
    exit 1
fi

echo "  Bootloader: $BOOT_EFI ($(du -h "$BOOT_EFI" | cut -f1))"
echo "  Kernel:     $KERNEL_ELF ($(du -h "$KERNEL_ELF" | cut -f1))"
echo ""

# Copy to ISO tree
echo "[COPY] Populating ISO tree..."
cp "$BOOT_EFI" iso/boot/efi/BOOTX64.EFI
cp "$KERNEL_ELF" iso/boot/bharat/kernel.elf

# Create a FAT32 disk image for EFI boot
echo "[DISK] Creating FAT32 EFI system partition..."
dd if=/dev/zero of=iso/boot/efi/esp.img bs=1M count=64 2>/dev/null
mkfs.fat -F 32 -n BHARATOS iso/boot/efi/esp.img >/dev/null 2>&1 || true

# Mount and populate ESP (requires sudo, so use mtools if available)
if command -v mcopy >/dev/null 2>&1; then
    echo "  Populating ESP with mtools..."
    mcopy -i iso/boot/efi/esp.img iso/boot/efi/BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI >/dev/null 2>&1 || true
fi

echo "  ISO tree ready."
ls -la iso/boot/efi/ iso/boot/bharat/
echo ""

# Build bootable ISO
echo "[ISO] Building bharatos.iso..."
# Try xorriso first
if command -v xorriso >/dev/null 2>&1; then
    xorriso -as mkisofs \
        -o bharatos.iso \
        -isohybrid-mbr /usr/lib/ISOLINUX/isohdpfx.bin \
        -c isolinux/boot.cat \
        -b isolinux/isolinux.bin \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        -eltorito-alt-boot \
        -e images/efiboot.img \
        -no-emul-boot \
        -isohybrid-gpt-basdat \
        iso/ 2>&1 | tail -5
else
    echo "WARNING: xorriso not found, creating zip archive instead"
    zip -r bharatos.iso.zip iso/
fi

if [ -f bharatos.iso ]; then
    echo "  ISO created: bharatos.iso ($(du -h bharatos.iso | cut -f1))"
elif [ -f bharatos.iso.zip ]; then
    echo "  Archive created: bharatos.iso.zip ($(du -h bharatos.iso.zip | cut -f1))"
else
    echo "ERROR: ISO creation failed"
    exit 1
fi

echo ""
echo "=== Build Complete ==="
echo "Artifacts:"
echo "  Bootloader EFI: $BOOT_EFI"
echo "  Kernel ELF:     $KERNEL_ELF"
echo "  ISO:            $PROJECT_ROOT/bharatos.iso"
echo ""
echo "Run: ./scripts/run.sh"
