# How to install the api extension for Zed

The `api-zed` extension embeds `api-core` directly and adds `/api` and
`/api-formats` assistant commands for converting OpenAPI specs.

Unlike the VS Code extension, the Zed extension is **not shipped as a prebuilt
archive** — `api-core` is a path dependency, so it must be built from a full
repository checkout. You need [Rust](https://rustup.rs/) installed.

## Option 1: install script (recommended)

```sh
curl -fsSL https://raw.githubusercontent.com/hieudoanm/api/master/scripts/install.sh | bash -- --zed
```

The script clones (or updates) the repository into `~/Downloads/api` and prints
the next steps. Then in Zed:

1. Open the command palette → **zed: install extension**.
2. Choose **Install Dev Extension**.
3. Select `~/Downloads/api/packages/extensions/zed`.

## Option 2: manual checkout

```sh
git clone https://github.com/hieudoanm/api.git
cd api/packages/extensions/zed
```

Then in Zed: command palette → **zed: install extension** → **Install Dev
Extension** → select that directory.

## Using it

- `/api <spec.yaml|json> [format]` — convert a spec to a client collection.
- `/api-formats` — list the supported output formats.

## Troubleshooting

- Build fails — install the Rust toolchain (`rustup`) and retry; the extension
  compiles `api-core` on first load.
- Dev extension missing after clone — make sure you selected
  `packages/extensions/zed` (the directory containing `extension.toml`), not the
  repository root.
