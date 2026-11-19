# AGENTS.md — api-core

Conversion engine for api. See the root [`AGENTS.md`](../../AGENTS.md).

- `src/openapi.rs` — OpenAPI 3.x parser (JSON or YAML via `serde_json` /
  `serde_yml` fallback): info, servers (variables resolved), paths →
  operations, parameters, request bodies (JSON/form/multipart/text),
  security schemes → `ApiAuth`.
- `src/model.rs` — wire types: `ApiCollection`/`ApiFolder`/`ApiRequest`,
  `ApiParameter`, `ApiBody`, `ApiAuth`, `ClientFormat` (+ `ALL`, `parse`,
  `as_str`, file-name helpers), `ConvertOptions`, `ConversionFile`,
  `ApiConvertError` (via `thiserror`).
- `src/convert.rs` — `convert_collection` / `convert_openapi` /
  `convert_openapi_all` dispatching on `ClientFormat`.
- `src/formats/` — one emitter per client: `postman.rs`, `bruno.rs`,
  `insomnia.rs`, `hoppscotch.rs`, `rest_client.rs`, `thunder_client.rs`,
  `yaak.rs`, `hurl.rs`, `kreya.rs`, `curl.rs`, plus shared helpers in
  `util.rs` (`unique_slug`, `auth_for_request`, media-type helpers).
- Pure library: no clap, no stdin/stdout, no file I/O — callers get
  `Result<Vec<ConversionFile>>` and decide what to do with the bytes.
- Adding a client format: new `formats/<name>.rs`, register in
  `formats/mod.rs`, add the `ClientFormat` variant + match arm in
  `convert.rs`, extend `parse`/`as_str`/`ALL`/file-name helpers in
  `model.rs`.
- Tests: `cargo test -p api-core`; keep coverage at or above 80% lines and
  functions — verify with `cargo llvm-cov -p api-core --summary-only`.
