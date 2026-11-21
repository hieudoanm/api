# AGENTS.md — packages/extensions/vscode

VS Code extension: **api: Convert OpenAPI Spec**. Converts OpenAPI 3.x JSON/YAML
specs to API client collections and HTML docs previews with the same `@api/wasm`
build used by the CLI, landing converter, and browser tools.

## Commands

```bash
pnpm build:wasm   # first time: builds crates/api-wasm wasm (wasm-pack)
pnpm build        # esbuild bundle → dist/extension.js + api_wasm_bg.wasm
pnpm typecheck    # tsc --noEmit
pnpm lint         # eslint
pnpm package      # → build/extension-vscode-api.vsix
```

## Layout

- `src/extension.ts` — the whole extension: command registration, spec pick
  (active editor or open dialog), format quick pick (`formats()` + `all`), wasm
  convert, write `api-collections/<format>/…` next to the spec
- `scripts/build.mjs` — esbuild CJS bundle; `vscode` stays external, `@api/wasm`
  is bundled, `api_wasm_bg.wasm` is copied to `dist/`
- `dist/` — build output (gitignored); `main` points here

## Gotchas

- Wasm init uses `initSync(bytes)` — never the default `init()`, which would try
  `fetch(new URL('api_wasm_bg.wasm', import.meta.url))` and fail in the CJS
  bundle.
- `package.json` `name` must be unscoped (`api-vscode`) — vsce rejects
  `@scope/*`.
- `vsce package` needs `--no-dependencies` (workspace `@api/wasm` is bundled,
  and npm can't resolve `workspace:*`), and the `--out` dir must exist first
  (`mkdir -p build`).
- `@types/vscode` must not exceed `engines.vscode` or vsce fails.
- `convert`/`convert_all`/`formats`/`openapi_info` return JSON **strings**;
  empty `name`/`base_url` args mean "unset".
