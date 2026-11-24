# api documentation

api converts [OpenAPI](https://spec.openapis.org/) specifications into
ready-to-use API client collections for Postman, Bruno, Insomnia, Hoppscotch,
REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — so your team can start
calling a real API in minutes instead of hand-building a collection request by
request.

api is a Rust workspace:

- `api-core` — the conversion engine: OpenAPI parsing, an intermediate
  collection model, and one emitter per client format.
- `api-cli` — the `api` command-line binary on top of `api-core`.
- `api-wasm` — WebAssembly bindings over `api-core`, consumed by the landing
  page's in-browser converter.

## Contents

### Getting started

- [Installation](getting-started/installation.md)
- [Quickstart](getting-started/quickstart.md)

### Concepts

- [OpenAPI support](concepts/openapi.md)
- [Collections & formats](concepts/collections.md)

### API reference

- [Output formats](api-reference/formats.md)
- [CLI reference](api-reference/cli.md)

### Guides

- [Using api in CI](guides/ci.md)

### Installation guides

- [CLI on macOS (Apple Silicon)](guides/how-to-install-cli-macos-silicon.md)
- [CLI on macOS (Intel)](guides/how-to-install-cli-macos-intel.md)
- [CLI on Linux](guides/how-to-install-cli-linux.md)
- [CLI on Windows](guides/how-to-install-cli-windows.md)
- [VS Code extension](guides/how-to-install-vscode.md)
- [Zed extension](guides/how-to-install-zed.md)
- [JetBrains plugin](guides/how-to-install-jetbrains.md)
