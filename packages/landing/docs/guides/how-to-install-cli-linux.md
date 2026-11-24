# How to install the api CLI on Linux

For x86_64 Linux distributions.

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash
```

The script detects `linux/x86_64` and downloads
`cli-api-x86_64-unknown-linux-gnu` from the
[latest release](https://github.com/hieudoanm/api/releases/latest). It installs
to `~/.local/bin/api` (creating the directory if needed).

## Option 2: manual download

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **Linux** under **CLI** — `cli-api-x86_64-unknown-linux-gnu`.
2. Make it executable and move it onto your `PATH`:

```sh
chmod +x cli-api-x86_64-unknown-linux-gnu
mv cli-api-x86_64-unknown-linux-gnu ~/.local/bin/api
```

3. Verify:

```sh
api --version
```

## Troubleshooting

- `api: command not found` — ensure `~/.local/bin` is on your `PATH` (most
  distributions already include it).
- `No such file or directory` on a glibc system — the binary is built against
  glibc; on musl-based distros (Alpine) build from source instead.
