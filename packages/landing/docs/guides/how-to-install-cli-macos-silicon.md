# How to install the api CLI on macOS (Apple Silicon)

For Macs with an M-series chip (arm64).

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash
```

The script detects `macos/arm64` and downloads `cli-api-aarch64-apple-darwin`
from the [latest release](https://github.com/hieudoanm/api/releases/latest). It
installs to `/opt/homebrew/bin/api` when Homebrew's bin is writable, otherwise
`/usr/local/bin/api` or `~/.local/bin/api`.

## Option 2: manual download

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **macOS (Silicon)** under **CLI** — `cli-api-aarch64-apple-darwin`.
2. Make it executable and move it onto your `PATH`:

```sh
chmod +x cli-api-aarch64-apple-darwin
mv cli-api-aarch64-apple-darwin /opt/homebrew/bin/api   # or ~/.local/bin/api
```

3. Verify:

```sh
api --version
```

## Troubleshooting

- `api: command not found` — the install directory is not on your `PATH`. Add
  `export PATH="$HOME/.local/bin:$PATH"` to your `~/.zshrc`.
- Gatekeeper warning — the binary is unsigned. Right-click → Open, or run
  `xattr -d com.apple.quarantine /opt/homebrew/bin/api`.
