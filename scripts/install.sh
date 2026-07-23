#!/usr/bin/env bash
set -euo pipefail

echo "=== BharatOS X — Environment Setup ==="
echo ""

# Detect distro
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo "Cannot detect Linux distribution"
    exit 1
fi

echo "Detected distro: $DISTRO"

# Install packages
echo "Installing build dependencies..."
case "$DISTRO" in
    ubuntu|debian|pop)
        sudo apt-get update -qq
        sudo apt-get install -y -qq \
            build-essential curl git ca-certificates \
            qemu-system-x86 ovmf xorriso mtools \
            pkg-config libssl-dev
        ;;
    arch|manjaro|endeavouros)
        sudo pacman -Sy --noconfirm \
            base-devel curl git qemu-ovmf-x86_64 \
            xorriso mtools openssl
        ;;
    fedora|rhel|centos)
        sudo dnf install -y \
            gcc make curl git \
        edk2-ovmf qemu-system-x86 xorriso mtools openssl-devel
        ;;
    *)
        echo "Unsupported distro: $DISTRO"
        echo "Install: build-essential, curl, qemu-system-x86, ovmf, xorriso, mtools"
        exit 1
        ;;
esac

# Install Rust via rustup (if missing)
if ! command -v cargo >/dev/null 2>&1; then
    echo "Installing Rust toolchain via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- -y --default-toolchain nightly --profile minimal
    echo 'source $HOME/.cargo/env' >> ~/.bashrc
    source "$HOME/.cargo/env"
fi

echo ""
echo "=== Installing Rust components ==="
rustup default nightly
rustup target add x86_64-unknown-uefi
rustup component add rust-src llvm-tools-preview

echo ""
echo "=== Verifying toolchain ==="
cargo --version
rustc --version
qemu-system-x86_64 --version | head -1

echo ""
echo "=== Setup Complete ==="
echo "To use cargo in this session: source ~/.cargo/env"
echo "Next: cd /home/basant/Code/OpratingSystem/BharatOS && ./scripts/build.sh && ./scripts/run.sh"
