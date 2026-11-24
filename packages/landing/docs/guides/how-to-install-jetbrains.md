# How to install the api plugin for JetBrains IDEs

The `api` JetBrains plugin (`com.api.idea`) adds api tooling inside IntelliJ
IDEA, GoLand, PyCharm and other JetBrains IDEs. It shells out to the `api` CLI,
so [install the CLI](../getting-started/installation.md) first (or set the
binary path in settings).

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash -- --jetbrains
```

The script downloads `extension-jetbrains-api.zip` from the
[latest release](https://github.com/hieudoanm/api/releases/latest) into
`~/Downloads`.

## Option 2: manual download

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **JetBrains** under **Editor Extensions** —
   `extension-jetbrains-api.zip`.

## Installing the zip

1. Open **Settings/Preferences → Plugins**.
2. Click the **⚙** gear icon → **Install Plugin from Disk…**.
3. Pick the downloaded `extension-jetbrains-api.zip` and restart the IDE if
   prompted.

## Configuring the CLI path

If `api` is not on your `PATH`, set it under **Settings → Tools → api** (or the
plugin's settings page) to the absolute path of the binary — for example
`/opt/homebrew/bin/api` or `~/.local/bin/api`.

## Troubleshooting

- Plugin installed but commands missing — the CLI is not found; install it or
  set the path as above.
- Update by repeating the download and **Install Plugin from Disk** again —
  JetBrains replaces the previous version.
