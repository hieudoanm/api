# How to install the api CLI on macOS (Intel)

For Intel Macs (x86_64).

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash
```

The script detects `macos/x86_64` and downloads `cli-api-x86_64-apple-darwin`
from the [latest release](https://github.com/hieudoanm/api/releases/latest). It
installs to `/usr/local/bin/api` when writable, otherwise `~/.local/bin/api`.

## Option 2: manual download

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **macOS (Intel)** under **CLI** — `cli-api-x86_64-apple-darwin`.
2. Make it executable and move it onto your `PATH`:

```sh
chmod +x cli-api-x86_64-apple-darwin
mv cli-api-x86_64-apple-darwin /usr/local/bin/api   # or ~/.local/bin/api
```

3. Verify:

```sh
api --version
```

## Troubleshooting

- `api: command not found` — add the install directory to your `PATH` in
  `~/.zshrc`.
- Gatekeeper warning — the binary is unsigned. Right-click → Open, or run
  `xattr -d com.apple.quarantine /usr/local/bin/api`.
