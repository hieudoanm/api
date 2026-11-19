# Contributing to api

Thanks for contributing to api, a Rust CLI that converts OpenAPI 3.x specs into
ready-to-import collections for Postman, Bruno, Insomnia, Hoppscotch, REST
Client, Thunder Client, Yaak, Hurl, Kreya and curl.

## Repository layout

- `crates/api-core` — conversion engine: OpenAPI parsing, the intermediate
  `ApiCollection` model, and one emitter per client format.
- `crates/api-cli` — thin `api` binary (clap) on top of `api-core`.
- `packages/landing` — Next.js landing page and docs site → GitHub Pages.

See the crate-level and package-level AGENTS.md files for per-component
conventions.

## Setup

```bash
git clone https://github.com/hieudoanm/api.git
cd api
cargo build --workspace
cargo run -p api-cli -- --help
```

## Checks

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

pnpm install
pnpm lint && pnpm typecheck && pnpm test   # JS workspace (packages/landing)
pnpm build                                  # static export → packages/landing/out
```

## Conventions

- Rust 2021 edition; dependency versions are pinned with `=` in crate
  manifests — match that style.
- `api-core` must stay UI/CLI-free: no clap, no stdout — it returns typed
  `Result`s; `api-cli` owns argument parsing and I/O.
- Errors are explicit typed errors (`ApiConvertError`) — no silent `unwrap()`
  on I/O or external input.
- New client formats: add an emitter under `crates/api-core/src/formats/`,
  register it in `formats/mod.rs` and `convert.rs` — the parser and model stay
  untouched.
- Do not commit secrets or `target/`.

## License

By contributing, you agree that your contributions will be licensed under the
repository license in [`LICENSE`](LICENSE).
