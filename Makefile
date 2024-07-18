# Define variables for common commands and paths
CARGO := cargo
RUSTUP := rustup
TAR := tar
TARGET_DIR := target

# Define targets for different platforms
DARWIN_TARGET := x86_64-apple-darwin
LINUX_TARGET := x86_64-unknown-linux-musl
WINDOWS_TARGET := x86_64-pc-windows-msvc
LINUX_ARM_TARGET := aarch64-unknown-linux-musl
DARWIN_ARM_TARGET := aarch64-apple-darwin

# Phony targets
.PHONY: all benchmark build build-for-mac build-for-linux build-for-windows release run run-build test test-integration lint lint-fix

# Default target
all: build

benchmark:
	$(CARGO) bench

build:
	$(CARGO) build

build-for-mac:
	$(RUSTUP) target add $(DARWIN_TARGET)
	$(CARGO) build --release --target $(DARWIN_TARGET)
	$(TAR) -czf loop-mac-x86_64.tar.gz -C $(TARGET_DIR)/$(DARWIN_TARGET)/release loop

build-for-linux:
	$(RUSTUP) target add $(LINUX_TARGET)
	$(CARGO) build --release --target $(LINUX_TARGET)
	$(TAR) -czf loop-linux-x86_64.tar.gz -C $(TARGET_DIR)/$(LINUX_TARGET)/release loop

build-for-windows:
	$(RUSTUP) target add $(WINDOWS_TARGET)
	$(CARGO) build --release --target $(WINDOWS_TARGET)
	$(TAR) -czf loop-windows-x86_64.tar.gz -C $(TARGET_DIR)/$(WINDOWS_TARGET)/release loop

build-for-linux-arm:
	$(RUSTUP) target add $(LINUX_ARM_TARGET)
	$(CARGO) build --release --target $(LINUX_ARM_TARGET)
	$(TAR) -czf loop-linux-aarch64.tar.gz -C $(TARGET_DIR)/$(LINUX_ARM_TARGET)/release loop

build-for-mac-arm:
	$(RUSTUP) target add $(DARWIN_ARM_TARGET)
	$(CARGO) build --release --target $(DARWIN_ARM_TARGET)
	$(TAR) -czf loop-mac-aarch64.tar.gz -C $(TARGET_DIR)/$(DARWIN_ARM_TARGET)/release loop

release:
	$(CARGO) build --release

run:
	$(CARGO) run

run-build:
	@if [ ! -f $(TARGET_DIR)/release/loop ]; then \
		echo "Release build not found. Building now..."; \
		$(MAKE) release; \
	fi
	./$(TARGET_DIR)/release/loop

test:
	$(CARGO) test --tests

test-integration:
	$(CARGO) test --test integration_tests

# Linting targets
lint:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy -- -D warnings

lint-fix:
	$(CARGO) fmt --all
	$(CARGO) clippy --fix --allow-dirty -- -D warnings

# Add a new target for the exec command
exec:
	@if [ ! -f $(TARGET_DIR)/debug/loop ]; then \
		echo "Debug build not found. Building now..."; \
		$(MAKE) build; \
	fi
	./$(TARGET_DIR)/debug/loop exec