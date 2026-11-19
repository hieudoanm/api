# api-wasm

WebAssembly bindings for [`api-core`](../api-core) — run OpenAPI→collection
conversion in the browser or any wasm host, no server required.

## Exported functions

All functions take/return JSON strings; errors are thrown as `JsError`.

| Function                                  | Description                                                                                                                                                                                        |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `formats()`                               | JSON array of supported client formats.                                                                                                                                                            |
| `openapi_info(source)`                    | Parse an OpenAPI 3.x doc (JSON/YAML) → summary: name, version, description, servers, folders, request count.                                                                                       |
| `convert(source, format, name, base_url)` | Convert to one format → `{"files": [{format, path, content}]}`. `format` is one of `postman`, `bruno`, `insomnia`, `hoppscotch`, `rest-client`, `thunder-client`, `yaak`, `hurl`, `kreya`, `curl`. |
| `convert_all(source, name, base_url)`     | Convert to every format at once.                                                                                                                                                                   |

Pass `""` for `name`/`base_url` to use the spec defaults.

## Build

```sh
rustup target add wasm32-unknown-unknown   # once
make build-wasm                            # or: pnpm --filter @api/wasm run build:wasm
```

`make build-wasm` builds the release `cdylib`
(`target/wasm32-unknown-unknown/release/api_wasm.wasm`) and runs
`wasm-bindgen --target web` into `pkg/`. The `pkg/` directory is gitignored;
`package.json` exposes it to pnpm as `@api/wasm`, and `packages/landing`
consumes it via `workspace:*`.

## Usage (JS)

```js
import init, { convert_all, formats } from '@api/wasm';

await init();
const spec = await fetch('openapi.yaml').then((r) => r.text());
const result = JSON.parse(convert_all(spec, '', ''));
// result.files -> [{ format, path, content }, ...]
```
