# AGENTS.md — api-cli

Thin `api` binary over `api-core` + `api-mcp`. See the root
[`AGENTS.md`](../../AGENTS.md).

- `src/main.rs` — clap derive CLI. Subcommands: `convert`, `formats`, `mcp`.
- `convert <input>` — input is a file path or `-` for stdin (no URL fetching).
  `-o/--out` (alias `--output`, default `./api-collections`), `-f/--format`
  comma-separated or `all` (default `postman`), `--name`, `--base-url`.
- Output layout: files land in `<out>/<format>/…` (`format.as_str()` dir).
- Diagnostics (`wrote …`) go to stderr; clap owns usage errors (exit 2) and
  help/version; runtime errors exit 1 via anyhow.
- `api mcp` runs the stdio MCP server from `api-mcp` — the CLI owns no MCP
  logic itself.
- Keep it thin: argument parsing + file/stdin I/O only; all conversion lives
  in `api-core`, all protocol handling in `api-mcp`.
- `cargo run -p api-cli -- --help`; `cargo test --workspace`; keep coverage
  at or above 80% lines and functions (`cargo llvm-cov -p api-cli`).
