.PHONY: all clean build kernel boot apps docs test check fmt iso qemu qemu-debug setup verify run install

CARGO ?= cargo
RUSTFLAGS_KERNEL ?= -Zbuild-std -Zbuild-std-features=std_detect_dl

export PATH := $(HOME)/.cargo/bin:$(PATH)

all: build

# Quick one-shot setup
install:
	@echo "[SETUP] Running installer..."
	@bash scripts/install.sh

setup:
	@echo "[SETUP] Alias for 'make install'"
	@bash scripts/install.sh

# Build entire workspace with release optimizations
build: kernel boot libs apps docs
	@echo "BharatOS X build complete"

# Build kernel with embedded profile
kernel:
	@echo "[KERNEL] Building BharatOS kernel..."
	$(CARGO) build --release -p bharat-kernel

# Build bootloader (UEFI)
boot:
	@echo "[BOOT] Building BharatOS bootloader..."
	$(CARGO) build --release -p bharat-boot --target x86_64-unknown-uefi

# Build libraries
libs: libcore libhal libmm libsched libfs libnet libcrypto libaep libsurface

libcore:
	$(CARGO) build --release -p bharat-libcore

libhal:
	$(CARGO) build --release -p bharat-libhal

libmm:
	$(CARGO) build --release -p bharat-libmm

libsched:
	$(CARGO) build --release -p bharat-libsched

libfs:
	$(CARGO) build --release -p bharat-libfs

libnet:
	$(CARGO) build --release -p bharat-libnet

libcrypto:
	$(CARGO) build --release -p bharat-libcrypto

libaep:
	$(CARGO) build --release -p bharat-libaep

libsurface:
	$(CARGO) build --release -p bharat-libsurface

# Build system utilities
sbin: bharat-init bharat-pkg bharat-fsck bharat-cryptsetup

bharat-init:
	$(CARGO) build --release -p bharat-init

bharat-pkg:
	$(CARGO) build --release -p bharat-pkg

bharat-fsck:
	$(CARGO) build --release -p bharat-fsck

bharat-cryptsetup:
	$(CARGO) build --release -p bharat-cryptsetup

# Build desktop environment
desktop: bharat-compositor satya-shell satya-theme boot-diagnostics

bharat-compositor:
	$(CARGO) build --release -p bharat-compositor

satya-shell:
	$(CARGO) build --release -p bharat-satya-shell

satya-theme:
	$(CARGO) build --release -p bharat-satya-theme

boot-diagnostics:
	$(CARGO) build --release -p bharat-boot-diagnostics

# Build first-party applications
apps: display-manager file-manager terminal editor \
      settings image-viewer bharat-browser music video camera \
      calculator calendar notes task-manager sound clock pdf-reader

display-manager:
	$(CARGO) build --release -p bharat-display-manager

file-manager:
	$(CARGO) build --release -p bharat-file-manager

terminal:
	$(CARGO) build --release -p bharat-terminal

editor:
	$(CARGO) build --release -p bharat-editor

settings:
	$(CARGO) build --release -p bharat-settings

image-viewer:
	$(CARGO) build --release -p bharat-image-viewer

bharat-browser:
	$(CARGO) build --release -p bharat-browser

music:
	$(CARGO) build --release -p bharat-music

video:
	$(CARGO) build --release -p bharat-video

camera:
	$(CARGO) build --release -p bharat-camera

calculator:
	$(CARGO) build --release -p bharat-calculator

calendar:
	$(CARGO) build --release -p bharat-calendar

notes:
	$(CARGO) build --release -p bharat-notes

task-manager:
	$(CARGO) build --release -p bharat-task-manager

sound:
	$(CARGO) build --release -p bharat-sound

clock:
	$(CARGO) build --release -p bharat-clock

pdf-reader:
	$(CARGO) build --release -p bharat-pdf-reader

# Build AI daemon
ai: bharat-ai voice-daemon

bharat-ai:
	$(CARGO) build --release -p bharat-ai

voice-daemon:
	$(CARGO) build --release -p bharat-voice-daemon

# Build developer tools
devtools: bharat-sdk bharat-debugger

bharat-sdk:
	$(CARGO) build --release -p bharat-sdk

bharat-debugger:
	$(CARGO) build --release -p bharat-debugger

# Build filesystem utilities
fs: mkfs-bharatfs mount

mkfs-bharatfs:
	$(CARGO) build --release -p mkfs-bharatfs

mount:
	$(CARGO) build --release -p bhfs

# Format code
fmt:
	$(CARGO) fmt --all

# Lint code
lint check:
	$(CARGO) check --workspace

# Run tests
test:
	$(CARGO) test --workspace

# Clean build artifacts
clean:
	$(CARGO) clean
	rm -rf target/
	rm -rf iso/
	rm -f bharatos.iso

# Build ISO image (requires ovmf, xorriso)
iso: build
	@echo "[ISO] Using scripts/build.sh..."
	@bash scripts/build.sh

# Run in QEMU
qemu: iso
	@echo "[QEMU] Using scripts/run.sh..."
	@bash scripts/run.sh

# Run in QEMU (debug)
qemu-debug: iso
	@echo "[QEMU] Using scripts/run.sh with GDB stub..."
	@GDB=1 bash scripts/run.sh
		-M q35 \
		-m 4G \
		-smp 4 \
		-cpu host \
		-S -s \
		-drive file=bharatos.iso,format=raw \
		-drive file=bharatos.img,format=raw \
		-netdev user,id=net0 \
		-device e1000,netdev=net0 \
		-serial stdio \
		-display gtk,gl=on

# Generate documentation
docs:
	@echo "[DOCS] Generating documentation..."
	mkdir -p docs/_build
	$(CARGO) doc --workspace --no-deps
	cp -r target/doc docs/_build/html

# Package for release
package: build docs
	@echo "[PKG] Creating release tarball..."
	tar czf bharatos-x-$(VERSION).tar.gz \
		--transform 's,^,bharatos-x-$(VERSION)/,' \
		target/ docs/ kernel/ boot/ libs/ apps/ desktop/ ai/ fs/ sbin/

# Run in QEMU (direct)
run:
	@bash scripts/run.sh

# Verify build integrity
verify:
	@bash scripts/verify.sh

# Check all
check-all: fmt check test verify
	@echo "All checks passed"

help:
	@echo "BharatOS X Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all            - Build everything (default)"
	@echo "  build          - Full build"
	@echo "  kernel         - Build kernel"
	@echo "  boot           - Build UEFI bootloader"
	@echo "  libs           - Build all libraries"
	@echo "  desktop        - Build desktop environment"
	@echo "  apps           - Build all applications"
	@echo "  ai             - Build AI daemons"
	@echo "  devtools       - Build SDK and debugger"
	@echo "  fs             - Build filesystem tools"
	@echo "  iso            - Build bootable ISO"
	@echo "  qemu           - Run in QEMU"
	@echo "  qemu-debug     - Run in QEMU with GDB"
	@echo "  verify         - Verify build integrity"
	@echo "  run            - Direct QEMU run"
	@echo "  install        - Install toolchain + dependencies"
	@echo "  docs           - Generate documentation"
	@echo "  package        - Create release tarball"
	@echo "  test           - Run tests"
	@echo "  check          - Check compilation"
	@echo "  fmt            - Format code"
	@echo "  clean          - Clean build artifacts"
	@echo "  help           - This help"
