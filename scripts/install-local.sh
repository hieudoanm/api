#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VSCODE_DIR="$REPO_ROOT/packages/extensions/vscode"
ZED_DIR="$REPO_ROOT/packages/extensions/zed"
JETBRAINS_DIR="$REPO_ROOT/packages/extensions/jetbrains"

ts() { date '+%Y-%m-%dT%H:%M:%S%z'; }

log()   { printf '[%s] [INFO ] %s\n' "$(ts)" "$*"; }
warn()  { printf '[%s] [WARN ] %s\n' "$(ts)" "$*" >&2; }
error() { printf '[%s] [ERROR] %s\n' "$(ts)" "$*" >&2; }
die()   { error "$*"; exit 1; }

usage() {
  cat <<'EOF'
Usage: install-local.sh [--cli|--vscode|--zed|--jetbrains] [DEST]

  --cli        Build the api CLI from source and install it (default)
  --vscode     Build the VS Code extension and install extension-vscode-api.vsix
  --zed        Build the Zed extension and point you at the source dir
               (Install Dev Extension — api-core is a path dependency)
  --jetbrains  Build the JetBrains plugin zip (./gradlew buildPlugin)

DEST is the install path for --cli, the download directory for --vscode
and --jetbrains, or ignored for --zed.

Environment:
  INSTALL_DIR  fallback install dir for --cli (default ~/.local/bin)
EOF
  exit "${1:-0}"
}

# --- Parse arguments -------------------------------------------------------
MODE=cli
DEST="${INSTALL_DIR:-}"
for arg in "$@"; do
  case "$arg" in
    --cli)        MODE=cli ;;
    --vscode)     MODE=vscode ;;
    --zed)        MODE=zed ;;
    --jetbrains)  MODE=jetbrains ;;
    -h|--help) usage 0 ;;
    --*)       die "Unknown flag: $arg" ;;
    *)         [ -z "$DEST" ] && DEST="$arg" || die "Unexpected argument: $arg" ;;
  esac
done

# --- Detect the OS ---------------------------------------------------------
case "$(uname -s)" in
  Linux*)               OS=linux ;;
  Darwin*)              OS=macos ;;
  MINGW*|MSYS*|CYGWIN*) OS=windows ;;
  *)                    die "Unsupported OS: $(uname -s)" ;;
esac
log "Detected OS: $OS, mode: $MODE"

# --- CLI -------------------------------------------------------------------
install_cli() {
  command -v cargo >/dev/null 2>&1 || die "cargo not found; install Rust from https://rustup.rs"
  log "cargo $(cargo --version | awk '{print $2}')"

  # Pick a writable install destination.
  local dest="$DEST"
  if [ -z "$dest" ]; then
    dest="$(command -v api 2>/dev/null || true)"
    if [ -n "$dest" ]; then
      log "Found existing install at $dest; will overwrite"
    elif [ "$OS" = "macos" ] && [ -d "/opt/homebrew/bin" ] && [ -w "/opt/homebrew/bin" ]; then
      dest="/opt/homebrew/bin/api"
    elif [ "$OS" = "macos" ] && [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
      dest="/usr/local/bin/api"
    elif [ -d "$HOME/.local/bin" ]; then
      mkdir -p "$HOME/.local/bin"
      dest="$HOME/.local/bin/api"
    elif [ "$OS" = "windows" ]; then
      dest="$HOME/bin/api.exe"
      mkdir -p "$HOME/bin"
    else
      die "Could not find a writable install directory; pass one explicitly."
    fi
  fi

  local dest_dir
  dest_dir="$(dirname "$dest")"
  mkdir -p "$dest_dir"
  [ -w "$dest_dir" ] || die "$dest_dir is not writable"

  log "Building release binary from $REPO_ROOT"
  (cd "$REPO_ROOT" && cargo build --release -p api-cli)

  local bin="$REPO_ROOT/target/release/api"
  [ "$OS" = "windows" ] && bin="$bin.exe"
  [ -x "$bin" ] || die "Build finished but $bin not found"
  log "Built $(wc -c < "$bin" | tr -d ' ') bytes at $bin"

  log "Installing to $dest"
  cp "$bin" "$dest"
  chmod +x "$dest"
  log "Installed api to $dest"

  if command -v "$dest" >/dev/null 2>&1; then
    log "Verifying install: $dest --version"
    "$dest" --version || true
  else
    log "Note: $dest is not on PATH yet. Add $dest_dir to your shell's PATH."
  fi
}

# --- VS Code (.vsix) --------------------------------------------------------
install_vscode() {
  command -v pnpm >/dev/null 2>&1 || die "pnpm not found; install pnpm 12+"
  log "Building VS Code extension (wasm + vsix)"
  (cd "$VSCODE_DIR" && pnpm build:wasm && pnpm package)
  local vsix="$VSCODE_DIR/build/extension-vscode-api.vsix"
  [ -f "$vsix" ] || die "Expected $vsix after pnpm package"

  local dir="$DEST"
  [ -z "$dir" ] && dir="$HOME/Downloads"
  mkdir -p "$dir"

  if command -v code >/dev/null 2>&1; then
    log "Installing extension: code --install-extension $vsix"
    if code --install-extension "$vsix"; then
      log "Installed api-vscode"
    else
      warn "code --install-extension failed; install $vsix manually"
      cp "$vsix" "$dir/extension-vscode-api.vsix"
      log "Copied $dir/extension-vscode-api.vsix"
    fi
  else
    cp "$vsix" "$dir/extension-vscode-api.vsix"
    log "Copied $dir/extension-vscode-api.vsix"
    cat <<EOF

Next steps (VS Code):
  The 'code' CLI was not found. Install the extension manually:
  1. Open VS Code → Extensions view (Cmd+Shift+X)
  2. Click "…" → "Install from VSIX…"
  3. Pick: $dir/extension-vscode-api.vsix
EOF
  fi
}

# --- Zed (source dir — api-core is a path dependency) ------------------------
install_zed() {
  command -v cargo >/dev/null 2>&1 || die "cargo not found; install Rust from https://rustup.rs"
  command -v make >/dev/null 2>&1 || die "make not found"
  rustup target list --installed | grep -q wasm32-wasip2 \
    || die "wasm32-wasip2 target missing; run: rustup target add wasm32-wasip2"
  log "Building Zed extension (wasm32-wasip2)"
  (cd "$ZED_DIR" && make build)
  [ -f "$ZED_DIR/extension.toml" ] || die "Expected $ZED_DIR/extension.toml"
  cat <<EOF

Next steps (Zed):
  1. Open the command palette → "zed: install extension"
  2. Choose "Install Dev Extension"
  3. Select: $ZED_DIR
  The extension links api-core by path, so it must build from this checkout.
  Then use /api <spec.yaml|json> [format] or /api-formats in the assistant panel.
EOF
}

# --- JetBrains (plugin zip) ---------------------------------------------------
install_jetbrains() {
  command -v java >/dev/null 2>&1 || die "java not found; install JDK 21+"
  log "Building JetBrains plugin (./gradlew buildPlugin)"
  (cd "$JETBRAINS_DIR" && ./gradlew buildPlugin --console=plain)
  local zip
  zip="$(find "$JETBRAINS_DIR/build/distributions" -name '*.zip' | head -1)"
  [ -f "$zip" ] || die "Expected a plugin zip under $JETBRAINS_DIR/build/distributions"

  local dir="$DEST"
  [ -z "$dir" ] && dir="$HOME/Downloads"
  mkdir -p "$dir"
  cp "$zip" "$dir/extension-jetbrains-api.zip"
  log "Copied $dir/extension-jetbrains-api.zip"
  cat <<EOF

Next steps (JetBrains):
  1. Open Settings → Plugins → ⚙ → "Install Plugin from Disk…"
  2. Pick: $dir/extension-jetbrains-api.zip
  The plugin shells out to the 'api' CLI — install it too
  (./scripts/install-local.sh --cli) or set the path in Settings → Tools → api.
EOF
}

case "$MODE" in
  cli)        install_cli ;;
  vscode)     install_vscode ;;
  zed)        install_zed ;;
  jetbrains)  install_jetbrains ;;
esac
