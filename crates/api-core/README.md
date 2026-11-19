# api-core

Core conversion engine for api: parses OpenAPI 3.x documents (JSON or YAML)
into a normalized `ApiCollection` model and emits it as API client collection
files for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl.

## Layout

| File             | What it is                                                                                                                       |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `src/openapi.rs` | OpenAPI 3.x parser — info, servers, paths/operations, parameters, request bodies, security schemes.                              |
| `src/model.rs`   | Wire types (`ApiCollection`, `ApiRequest`, `ApiAuth`, `ClientFormat`, `ConvertOptions`, `ConversionFile`) and `ApiConvertError`. |
| `src/convert.rs` | `convert_collection` / `convert_openapi` / `convert_openapi_all` — format dispatch.                                              |
| `src/formats/`   | One emitter per client format + shared helpers (`util.rs`).                                                                      |

## Usage

```rust
use api_core::{convert_openapi, ClientFormat, ConvertOptions};

let files = convert_openapi(spec_yaml_or_json, ClientFormat::Postman, &ConvertOptions::default())?;
for file in files {
    // file.format, file.path, file.content
}
```

`parse_openapi(source)` gives the intermediate `ApiCollection` if you need to
inspect the parsed model without converting.

## Errors

All failures are `ApiConvertError` variants: `Parse`, `Invalid`,
`UnsupportedVersion` (non-3.x specs), `Conversion`.
