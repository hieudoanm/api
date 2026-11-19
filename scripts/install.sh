#!/usr/bin/env bash
#
# install.sh — Install api from the latest GitHub release, or build from
# source if no prebuilt binary is published.
#
# Usage:
#   ./scripts/install.sh
#   INSTALL_DIR=/usr/local/bin ./scripts/install.sh
set -euo pipefail

REPO="hieudoanm/api"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "Installing api..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64 | amd64) ARCH="x86_64" ;;
    arm64 | aarch64) ARCH="aarch64" ;;
esac

case "$OS" in
    linux)  TRIPLE="${ARCH}-unknown-linux-gnu" ;;
    darwin) TRIPLE="${ARCH}-apple-darwin" ;;
    *)      TRIPLE="x86_64-pc-windows-msvc" ;;
esac

BINARY_URL="https://github.com/${REPO}/releases/latest/download/api-${TRIPLE}"
[[ "$OS" != "linux" && "$OS" != "darwin" ]] && BINARY_URL="${BINARY_URL}.exe"

if command -v curl >/dev/null 2>&1; then
    if curl -fsSL "$BINARY_URL" -o /tmp/api 2>/dev/null; then
        chmod +x /tmp/api
        mkdir -p "$INSTALL_DIR"
        install -m 0755 /tmp/api "$INSTALL_DIR/api"
        rm -f /tmp/api
        echo "api installed to ${INSTALL_DIR}/api"
        "$INSTALL_DIR/api" --help >/dev/null
        exit 0
    fi
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust toolchain not found. Please install Rust from https://rustup.rs"
    exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
cd "$TMP_DIR"
git clone --depth 1 "https://github.com/${REPO}.git" api
cd api
cargo build --release -p api-cli
mkdir -p "$INSTALL_DIR"
install -m 0755 target/release/api "$INSTALL_DIR/api"
echo "api installed to ${INSTALL_DIR}/api"
"$INSTALL_DIR/api" --help >/dev/null
