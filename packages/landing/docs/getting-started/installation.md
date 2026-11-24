# Installation

Step-by-step guides per platform and editor live under [Guides](../guides/) —
CLI for [macOS (Apple Silicon)](../guides/how-to-install-cli-macos-silicon.md),
[macOS (Intel)](../guides/how-to-install-cli-macos-intel.md),
[Linux](../guides/how-to-install-cli-linux.md) and
[Windows](../guides/how-to-install-cli-windows.md), plus the
[VS Code](../guides/how-to-install-vscode.md),
[Zed](../guides/how-to-install-zed.md) and
[JetBrains](../guides/how-to-install-jetbrains.md) extensions.

## Prebuilt binaries

Prebuilt binaries for Linux, macOS and Windows are attached to each
[release](https://github.com/hieudoanm/api/releases). Download the archive for
your platform, unpack it and put `api` on your `PATH`.

## Cargo

Once the crate is published:

```sh
cargo install api-cli
```

Until then, build from source:

```sh
cargo install --git https://github.com/hieudoanm/api api-cli
```

or clone the repository and build the workspace:

```sh
git clone https://github.com/hieudoanm/api
cd api
cargo build --release -p api-cli
./target/release/api --version
```

## Requirements

- Rust 1.85+ (edition 2021 workspace) for building from source
- Any OpenAPI 3.x document — JSON or YAML
