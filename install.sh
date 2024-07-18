#!/bin/bash

set -e

GITHUB_USER="mateodelnorte"
REPO_NAME="loop"
BINARY_NAME="loop"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Handle macOS on Apple Silicon
if [ "$OS" = "darwin" ] && [ "$ARCH" = "aarch64" ]; then
    OS="mac"
fi

# Determine latest version
echo "Fetching latest version..."
LATEST_VERSION=$(curl -s https://api.github.com/repos/$GITHUB_USER/$REPO_NAME/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "Failed to fetch latest version. Please check your internet connection and try again."
    exit 1
fi

echo "Latest version: $LATEST_VERSION"

# Construct download URL
DOWNLOAD_URL="https://github.com/$GITHUB_USER/$REPO_NAME/releases/download/$LATEST_VERSION/${BINARY_NAME}-${OS}-${ARCH}.tar.gz"

# Download and extract
echo "Downloading $BINARY_NAME..."
curl -L "$DOWNLOAD_URL" | tar xz -C /tmp

# Install binary
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
sudo mv "/tmp/$BINARY_NAME" "$INSTALL_DIR/"

# Set permissions
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "$BINARY_NAME $LATEST_VERSION has been installed to $INSTALL_DIR"
echo "You may need to restart your terminal or source your shell configuration file to use the 'loop' command."