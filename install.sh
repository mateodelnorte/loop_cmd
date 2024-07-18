#!/bin/bash

set -e

GITHUB_USER="mateodelnorte"
REPO_NAME="loop_cmd"
BINARY_NAME="loop"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
if [ "$OSTYPE" = "msys" ] || [ "$OSTYPE" = "win32" ]; then
    OS="windows"
else
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
fi
ARCH=$(uname -m)

# Map architecture names
case $OS in
    linux)
        ARCH="x86_64"
        ;;
    darwin)
        if [ "$ARCH" = "arm64" ]; then
            ARCH="arm64"
        else
            ARCH="x86_64"
        fi
        ;;
    windows)
        ARCH="x86_64"
        BINARY_NAME="loop.exe"
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Determine latest version
echo "Fetching latest version..."
LATEST_VERSION=$(curl -s https://api.github.com/repos/$GITHUB_USER/$REPO_NAME/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "Failed to fetch latest version. Please check your internet connection and try again."
    exit 1
fi

echo "Latest version: $LATEST_VERSION"

# Construct download URL
DOWNLOAD_URL="https://github.com/$GITHUB_USER/$REPO_NAME/releases/download/$LATEST_VERSION/${BINARY_NAME}-${OS}-${ARCH}"
if [ "$OS" = "windows" ]; then
    DOWNLOAD_URL="${DOWNLOAD_URL}.exe"
fi

# Download and install
echo "Downloading $BINARY_NAME..."
curl -L -o "$BINARY_NAME" "$DOWNLOAD_URL"

if [ ! -f "$BINARY_NAME" ]; then
    echo "Failed to download ${BINARY_NAME}. Please check your internet connection and try again."
    exit 1
fi

echo "Installing $BINARY_NAME to $INSTALL_DIR..."
if [ "$OS" = "windows" ]; then
    mkdir -p "$INSTALL_DIR"
    mv "$BINARY_NAME" "$INSTALL_DIR"
else
    chmod +x "$BINARY_NAME"
    sudo mv "$BINARY_NAME" "$INSTALL_DIR"
fi

echo "$BINARY_NAME has been installed successfully!"
echo "You can now use it by running 'loop' in your terminal."