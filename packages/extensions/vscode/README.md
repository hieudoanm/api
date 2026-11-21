# api — VS Code extension

Convert OpenAPI 3.x specs (JSON or YAML) into ready-to-import API client
collections — Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client,
Yaak, Hurl, Kreya, curl, plus standalone HTML docs previews (Swagger UI, Redoc,
Scalar, Stoplight, RapiDoc) — powered by the same
[`@api/wasm`](../../../crates/api-wasm) build used by the CLI, landing page
converter, and browser app.

## Usage

- **Explorer** — right-click a `.yaml`/`.yml`/`.json` spec → **Convert OpenAPI
  Spec**
- **Editor** — right-click inside an open spec, or use the "…" menu on its tab
- **Command Palette** — `api: Convert OpenAPI Spec` converts the active spec, or
  opens a file picker when nothing spec-like is focused

Pick a target format (or `all`) from the quick pick. Output is written next to
the spec under `api-collections/<format>/…`, mirroring the `api` CLI layout.

## Development

```bash
pnpm install
pnpm build:wasm   # first time only: builds crates/api-wasm wasm
pnpm build        # esbuild bundle → dist/extension.js + api_wasm_bg.wasm
pnpm typecheck
pnpm lint
```

Press <kbd>F5</kbd> in VS Code to launch an Extension Development Host with the
extension loaded from `dist/`.

## Package

```bash
pnpm package   # → build/extension-vscode-api.vsix
```

## Layout

- `src/extension.ts` — command registration, spec pick, format quick pick, wasm
  convert, write `api-collections/` next to the spec
- `scripts/build.mjs` — esbuild bundle (CJS) + wasm copy
- `dist/` — build output (gitignored)
