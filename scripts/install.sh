#!/usr/bin/env bash
set -euo pipefail

BASE_URL="https://github.com/hieudoanm/api/releases/download/latest"

ts() { date '+%Y-%m-%dT%H:%M:%S%z'; }

log()   { printf '[%s] [INFO ] %s\n' "$(ts)" "$*"; }
warn()  { printf '[%s] [WARN ] %s\n' "$(ts)" "$*" >&2; }
error() { printf '[%s] [ERROR] %s\n' "$(ts)" "$*" >&2; }
die()   { error "$*"; exit 1; }

usage() {
  cat <<'EOF'
Usage: install.sh [--cli|--vscode|--zed] [DEST]

  --cli       Install the api CLI binary (default)
  --vscode    Download the VS Code extension (extension-vscode-api.vsix) and install it
  --zed       Clone the repo and point you at the Zed dev extension
              (packages/extensions/zed — api-core is a path dependency, so the
              extension must be built from a full checkout)

DEST is the install path for --cli, the download directory for --vscode,
or the parent directory for --zed.

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
    --cli)     MODE=cli ;;
    --vscode)  MODE=vscode ;;
    --zed)     MODE=zed ;;
    -h|--help) usage 0 ;;
    --*)       die "Unknown flag: $arg" ;;
    *)         [ -z "$DEST" ] && DEST="$arg" || die "Unexpected argument: $arg" ;;
  esac
done

# --- Detect the OS ---------------------------------------------------------
case "$(uname -s)" in
  Linux*)                 OS=linux ;;
  Darwin*)                OS=macos ;;
  MINGW*|MSYS*|CYGWIN*)   OS=windows ;;
  *)                      die "Unsupported OS: $(uname -s)" ;;
esac
ARCH="$(uname -m)"
log "Detected OS: $OS ($ARCH), mode: $MODE"

download() { # url dest
  log "Downloading $1"
  curl -fsSL --retry 3 --retry-delay 2 "$1" -o "$2"
  log "Downloaded $(wc -c < "$2" | tr -d ' ') bytes"
}

# --- CLI binary ------------------------------------------------------------
install_cli() {
  local asset
  case "$OS/$ARCH" in
    macos/arm64)   asset=cli-api-aarch64-apple-darwin ;;
    linux/x86_64)  asset=cli-api-x86_64-unknown-linux-gnu ;;
    windows/*)     asset=cli-api-x86_64-pc-windows-msvc.exe ;;
    *)             die "No release asset for $OS/$ARCH; build from source with scripts/install-local.sh" ;;
  esac
  local url="$BASE_URL/$asset"

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

  local tmp
  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' EXIT
  download "$url" "$tmp"

  log "Installing to $dest"
  mv "$tmp" "$dest"
  chmod +x "$dest" 2>/dev/null || true
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
  local dir="$DEST"
  [ -z "$dir" ] && dir="$HOME/Downloads"
  mkdir -p "$dir"
  local vsix="$dir/extension-vscode-api.vsix"
  download "$BASE_URL/extension-vscode-api.vsix" "$vsix"

  if command -v code >/dev/null 2>&1; then
    log "Installing extension: code --install-extension $vsix"
    if code --install-extension "$vsix"; then
      log "Installed api-vscode"
    else
      warn "code --install-extension failed; install $vsix manually"
    fi
  else
    cat <<EOF

Next steps (VS Code):
  The 'code' CLI was not found. Install the extension manually:
  1. Open VS Code → Extensions view (Cmd+Shift+X)
  2. Click "…" → "Install from VSIX…"
  3. Pick: $vsix
EOF
  fi
}

# --- Zed (source checkout — api-core is a path dependency) -------------------
install_zed() {
  command -v git >/dev/null 2>&1 || die "git not found; install git first"
  local dir="$DEST"
  [ -z "$dir" ] && dir="$HOME/Downloads"
  mkdir -p "$dir"
  local dest="$dir/api"
  if [ -d "$dest/.git" ]; then
    log "Found existing checkout at $dest; pulling latest"
    git -C "$dest" pull --ff-only || warn "git pull failed; using existing checkout"
  else
    log "Cloning https://github.com/hieudoanm/api.git into $dest"
    git clone --depth 1 https://github.com/hieudoanm/api.git "$dest"
  fi
  local zed_dir="$dest/packages/extensions/zed"
  [ -f "$zed_dir/extension.toml" ] || die "Expected $zed_dir/extension.toml in the checkout"
  cat <<EOF

Next steps (Zed):
  1. Open the command palette → "zed: install extension"
  2. Choose "Install Dev Extension"
  3. Select: $zed_dir
  The extension links api-core by path, so it must build from this checkout.
  Then use /api <spec.yaml|json> [format] or /api-formats in the assistant panel.
EOF
}

case "$MODE" in
  cli)     install_cli ;;
  vscode)  install_vscode ;;
  zed)     install_zed ;;
esac
