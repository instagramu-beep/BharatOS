#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

ISO="${1:-$PROJECT_ROOT/bharatos.iso}"
DISK="${2:-$PROJECT_ROOT/bharatos.img}"

if [ ! -f "$ISO" ]; then
    echo "ISO not found: $ISO"
    echo "Run ./scripts/build.sh first"
    exit 1
fi

echo "=== BharatOS X — QEMU Runtime ==="
echo ""

# Detect OVMF
OVMF_CODE=""
OVMF_VARS=""

for candidate in \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/qemu/OVMF_CODE.fd \
    /usr/share/edk2-ovmf/x64/OVMF_CODE.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd; do
    if [ -f "$candidate" ]; then
        OVMF_CODE="$candidate"
        break
    fi
done

for candidate in \
    /usr/share/OVMF/OVMF_VARS.fd \
    /usr/share/qemu/OVMF_VARS.fd \
    /usr/share/edk2-ovmf/x64/OVMF_VARS.fd \
    /usr/share/edk2/x64/OVMF_VARS.fd; do
    if [ -f "$candidate" ]; then
        OVMF_VARS="$candidate"
        break
    fi
done

if [ -z "$OVMF_CODE" ] || [ -z "$OVMF_VARS" ]; then
    echo "ERROR: OVMF firmware not found"
    echo "Install ovmf package and try again"
    exit 1
fi

echo "OVMF_CODE: $OVMF_CODE"
echo "OVMF_VARS: $OVMF_VARS"
echo "ISO:       $ISO"
echo ""

# Create disk image if missing
if [ ! -f "$DISK" ]; then
    echo "Creating disk image: $DISK (4G)"
    qemu-img create -f qcow2 "$DISK" 4G
fi

# Clean previous runtime state
rm -f /tmp/bharatos-ovmf-vars.fd || true
cp "$OVMF_VARS" /tmp/bharatos-ovmf-vars.fd

echo "[QEMU] Launching BharatOS..."
echo "  Machine: q35"
echo "  CPUs:    4 (host)"
echo "  RAM:     4G"
echo "  Serial:  stdio"
echo "  Display: gtk+gl"
echo ""
echo "Press Ctrl+A then X to exit QEMU"
echo ""

qemu-system-x86_64 \
    -machine q35 \
    -cpu host \
    -smp 4 \
    -m 4G \
    -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
    -drive if=pflash,format=raw,file=/tmp/bharatos-ovmf-vars.fd \
    -drive file="$ISO",format=raw,media=cdrom \
    -drive file="$DISK",format=qcow2 \
    -netdev user,id=net0 \
    -device e1000,netdev=net0 \
    -serial stdio \
    -display gtk,gl=on \
    -enable-kvm \
    -device virtio-gpu-gl \
    -device virtio-balloon \
    -rtc base=localtime \
    -no-reboot

echo ""
echo "QEMU exited."
