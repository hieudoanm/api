# How to install the api CLI on Windows

For 64-bit Windows.

The release binary is a native `.exe`. The install script is bash, so on Windows
run it from **Git Bash** or **WSL** — or download the binary manually.

## Option 1: install script via Git Bash or WSL

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash
```

The script detects Windows and downloads `cli-api-x86_64-pc-windows-msvc.exe`,
installing it to `~/bin/api.exe` (or the path you pass as `DEST`).

## Option 2: manual download

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **Windows** under **CLI** — `cli-api-x86_64-pc-windows-msvc.exe`.
2. Rename it to `api.exe` and place it in a directory on your `PATH` (for
   example `%LOCALAPPDATA%\Programs\api\` or `C:\tools\`).
3. Open a new terminal and verify:

```powershell
api --version
```

## Troubleshooting

- `api is not recognized` — the directory is not on `PATH`. Add it via Settings
  → Environment Variables, or call the binary by full path.
- SmartScreen warning — the binary is unsigned. Choose **More info → Run
  anyway**.
