# How to install the api extension for VS Code

The `api-vscode` extension adds commands and a sidebar tree for converting
OpenAPI specs without leaving the editor.

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash -- --vscode
```

The script downloads `extension-vscode-api.vsix` from the
[latest release](https://github.com/hieudoanm/api/releases/latest) and runs
`code --install-extension` when the `code` CLI is available.

## Option 2: manual install from the downloads page

1. Open the [downloads page](https://hieudoanm.github.io/api/downloads/) and
   download **VS Code** under **Editor Extensions** —
   `extension-vscode-api.vsix`.
2. In VS Code, open the Extensions view (`Cmd+Shift+X` / `Ctrl+Shift+X`).
3. Click **… → Install from VSIX…** and pick the downloaded file.

## Verifying

Open the Command Palette (`Cmd+Shift+P`) and run an `api:` command, or open a
spec file and use the api sidebar view.

## Troubleshooting

- The extension needs no separate runtime — conversion runs in the editor via
  the bundled WebAssembly build.
- To update, re-run the install script or install the newer `.vsix` the same
  way; VS Code replaces the existing version.
