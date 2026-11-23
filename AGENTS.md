# AGENTS.md — api

Guidance for AI agents working in this repository.

## What this is

api converts OpenAPI 3.x specifications (JSON or YAML) into ready-to-import
API client collections for Postman, Bruno, Insomnia, Hoppscotch, REST
Client, Thunder Client, Yaak, Hurl, Kreya and curl. Inspired by
hand-rolled Postman imports; see
[`README.md`](./README.md).

## Repository layout

| Path                         | What it is                                                                                                                                       |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `crates/api-core`            | Conversion engine: `openapi.rs` (parser), `model.rs` (wire types), `formats/` (one emitter per client), `convert.rs`.                            |
| `crates/api-cli`             | Thin binary: `api` — clap CLI over `api-core`.                                                                                                   |
| `crates/api-mcp`             | MCP stdio server exposing the converter tools.                                                                                                   |
| `crates/api-wasm`            | `#[wasm_bindgen]` exports over `api-core`; also a pnpm workspace package (`@api/wasm`) whose generated `pkg/` is consumed by the landing app.    |
| `packages/landing`           | Next.js + Tailwind/DaisyUI landing page **and** docs site (`/docs/*`) → GitHub Pages. See `packages/landing/AGENTS.md`.                          |
| `packages/extensions/vscode` | VS Code extension (`api-vscode`) over `@api/wasm`. See its `AGENTS.md`.                                                                          |
| `packages/extensions/zed`    | Zed extension (`api-zed`) embedding `api-core` directly; standalone crate (workspace `exclude`), built for `wasm32-wasip2`. See its `AGENTS.md`. |
| `scripts/`                   | `install.sh` (prebuilt release or cargo fallback) + `install-local.sh` (build from this checkout).                                               |
| `.github/workflows/ci.yaml`  | Rust check + CLI release binaries + Pages deploy.                                                                                                |

## Commands

```bash
cargo build --workspace          # debug build
cargo test --workspace           # all tests
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

JS workspace (pnpm — `packages/landing` plus the `@api/wasm` crate package):

```bash
pnpm install
pnpm --filter @api/wasm run build:wasm   # generate crates/api-wasm/pkg (gitignored)
pnpm dev            # landing dev server → :3000/api/
pnpm build          # static export → packages/landing/out
pnpm lint && pnpm typecheck && pnpm test
```

Run locally: `cargo run -p api-cli -- convert openapi.yaml --format all`.

## Conventions

- **Rust 2021 edition**; dependency versions are pinned with `=` in crate
  manifests — match that style when adding deps; prefer versions published
  > 7 days ago.
- Errors are explicit: `api-core` returns typed `ApiConvertError`s — no silent
  `unwrap()` on I/O or external input.
- `api-core` must not depend on clap/stdout concerns; `api-cli` owns argument
  parsing and I/O. Keep the dependency graph acyclic.
- Keep test coverage at or above 80% lines and functions for new and changed
  code — Rust: `cargo llvm-cov -p <crate> --summary-only`; landing:
  `pnpm --filter @api/landing run test:coverage` (vitest enforces 80% on
  statements, branches, functions and lines).
- New client formats: add `crates/api-core/src/formats/<name>.rs`, register in
  `formats/mod.rs` and `convert.rs`, extend `ClientFormat` in `model.rs`.
- Docs live in `packages/landing/docs/` as Markdown — update them when the
  CLI surface or formats change.
- Release binaries are named `cli-api-<target-triple>` (see `ci.yaml`
  `cli-build`); `scripts/install.sh` and `RELEASE.md` depend on that naming.
- Do not commit secrets or `target/`.
