#!/bin/sh
set -e

VERSION=$(curl -s https://api.github.com/repos/Hatya-mouse/kaslc/releases/latest | grep '"tag_name"' | cut -d'"' -f4)

ARCH=$(uname -m)
OS=$(uname -s)

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
elif [ "$OS" = "Linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

curl -L "https://github.com/Hatya-mouse/kaslc/releases/download/${VERSION}/kaslc-${TARGET}.tar.gz" | tar -xz
sudo mv kaslc /usr/local/bin/
echo "✓ Installed kaslc ${VERSION}"
