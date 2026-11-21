# api — latest build

![api — OpenAPI to API client collections](assets/branding/png/og.png)

api converts OpenAPI 3.x specs (JSON or YAML) into ready-to-import collections
for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — or into
standalone browsable API docs pages (Swagger UI, ReDoc, Scalar, Stoplight
Elements, RapiDoc) via `api preview`. One Rust CLI, no
cloud, no accounts.

### Download

| Platform            | Asset                                |
| ------------------- | ------------------------------------ |
| Linux x86_64        | `cli-api-x86_64-unknown-linux-gnu`   |
| macOS Apple Silicon | `cli-api-aarch64-apple-darwin`       |
| Windows x86_64      | `cli-api-x86_64-pc-windows-msvc.exe` |
| VS Code extension   | `extension-vscode-api.vsix`          |
| Zed extension       | `extension-zed-api.tar.gz`           |

Or install from source:

```sh
./scripts/install.sh --cli      # release binary
./scripts/install.sh --vscode   # VS Code extension (.vsix)
./scripts/install.sh --zed      # Zed extension (repo checkout)
./scripts/install-local.sh      # build everything from this checkout
cargo install api-cli           # once published
```

### Quick start

```sh
api convert openapi.yaml --format all --out ./collections
api preview openapi.yaml --format all --out ./docs
```

### Documentation

- [README](https://github.com/hieudoanm/api#readme)
- [Docs site](https://hieudoanm.github.io/api/docs/)
