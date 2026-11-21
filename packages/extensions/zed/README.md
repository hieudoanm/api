# api — Zed extension

Convert OpenAPI 3.x specs (JSON or YAML) into API client collections from
inside [Zed](https://zed.dev), powered by [`api-core`](../../../crates/api-core)
compiled into the extension itself.

Zed's extension API currently has no file context-menu or command-palette
surface, and worktrees are read-only for extensions — so this package exposes
assistant **slash commands** that return the converted files' contents as
text:

```
/api <spec.yaml|json> [format|all]
/api-formats
```

The spec path is worktree-relative. The second argument is a client format
(`postman`, `bruno`, `insomnia`, `hoppscotch`, `rest-client`, `thunder-client`,
`yaak`, `hurl`, `kreya`, `curl`) or a preview format (`swagger-ui`, `redoc`,
`scalar`, `stoplight`, `rapidoc`); it defaults to `postman`, and `all` emits
every format. Run `/api-formats` for the live list — argument completion also
offers format names.

Each generated file is emitted as a fenced block under its output path
(`<format>/<file>`), ready to be written by the assistant or copied by hand.

## How it works

Unlike `vectors-zed` (which shells out to a CLI), this extension links
`api-core` by path and converts in-process — no external binary is needed.
The trade-off: Zed's extension API cannot write into the worktree, so results
are delivered as slash-command text rather than files on disk.

## Build

Requires the `wasm32-wasip2` Rust target:

```bash
rustup target add wasm32-wasip2
make build      # cargo build --release --target wasm32-wasip2
make package    # → download/extension-zed-api.tar.gz
```

`make lint` runs `cargo clippy -D warnings`; `make format` runs `cargo fmt`.

## Install in Zed

Zed builds extensions from source. Because `api-core` is a path dependency, the
extension must be built from this repository checkout:

1. `zed: install extension` in the command palette
2. Choose **Install Dev Extension**
3. Select `packages/extensions/zed` (must contain `extension.toml`,
   `Cargo.toml`, `src/`)

Or copy the directory into `~/.config/zed/extensions/installed/api/` — the
`api-core` path dependency still resolves relative to the repo layout, so keep
the checkout intact.

For registry distribution the extension would be added as a git submodule of
[zed-industries/extensions](https://github.com/zed-industries/extensions) — at
that point `api-core` would need to be published to crates.io or vendored.

## Notes

- `Cargo.lock` is committed so Zed's source build stays reproducible.
- If upstream Zed adds a worktree write surface (or replaces slash commands
  with MCP/context servers), this package can move to writing real files or
  exposing conversion as MCP tools.
