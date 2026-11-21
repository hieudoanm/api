# AGENTS.md — `packages/extensions/zed`

Zed extension for api: a Rust crate compiled to `wasm32-wasip2` that embeds
`api-core` (path dependency) and exposes assistant slash commands. Unlike
`vectors-zed` there is no CLI subprocess — conversion runs in-process.

## Layout

| Path             | Holds                                                             |
| ---------------- | ----------------------------------------------------------------- |
| `extension.toml` | Zed manifest: id, metadata, `[slash_commands.*]`                  |
| `Cargo.toml`     | cdylib crate; deps `zed_extension_api` + `api-core` (path)        |
| `src/lib.rs`     | `Extension` impl: `run_slash_command` → `api_core` convert → text |
| `Makefile`       | `build` / `package` / `lint` / `format` / `clean`                 |
| `download/`      | `extension-zed-api.tar.gz` release artifact (gitignored)          |

## Rules

- Build target is **`wasm32-wasip2`** (not `wasip1`, not
  `wasm32-unknown-unknown`). `make build`/`make package` handle it.
- Keep `Cargo.lock` committed — Zed builds from source and the lockfile keeps
  `zed_extension_api`/`api-core` deps reproducible.
- `api-core` is a **path dependency** (`../../../crates/api-core`), so the
  extension only builds inside this monorepo checkout. If it is ever
  submoduled into zed-industries/extensions, `api-core` must be published or
  vendored first.
- Zed's extension API cannot write into the worktree — converted files are
  returned as slash-command text (fenced code blocks). Do not invent a write
  surface; if upstream adds one (or an MCP/context-server surface), migrate.
- `ClientFormat::ALL` includes client **and** preview formats; `/api` accepts
  any of them plus `all` (via `convert_openapi_all`).
- Release asset name is fixed: `extension-zed-api.tar.gz`.
