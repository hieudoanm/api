# AGENTS.md — api-mcp

MCP (Model Context Protocol) server exposing api's OpenAPI→collection
conversion as tools over stdio. Part of the `api-*` workspace.

## Conventions

- Newline-delimited JSON-RPC 2.0, protocol `2024-11-05` — matches what MCP
  clients expect: `initialize` → `notifications/initialized` → `tools/list` →
  `tools/call`.
- Core logic lives in `handle_message`/`call_tool` (pure, testable); `serve()`
  is only the stdio loop — nothing but frames on stdout.
- Only external dep is `serde_json` (protocol). All real work goes through
  `api-core` (`parse_openapi`, `convert_openapi`, `ClientFormat`) — no
  duplicated conversion logic.
- Tool names: `list_formats`, `openapi_info`, `convert_openapi` — keep them
  stable; MCP clients cache schemas.
- `convert_openapi` returns generated files as `{format, path, content}`
  JSON — it never writes to disk; the caller decides where output lands.
- Spec input is `source` (inline text) or `path` (file on disk) — there is no
  URL fetching, matching the CLI.

## Validation

```bash
cd crates
cargo test -p api-mcp
cargo llvm-cov -p api-mcp --summary-only   # lines+functions >= 80%
cargo clippy -p api-mcp
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | ./target/debug/api mcp
```
