# CLI reference

```text
api convert <spec> [options]
api preview <spec> [options]
```

`convert` emits API client collections; `preview` emits standalone HTML
documentation pages. Each command only accepts formats from its own group —
`convert` rejects preview formats and vice versa.

## Arguments

| Argument | Description                                                      |
| -------- | ---------------------------------------------------------------- |
| `<spec>` | Path to an OpenAPI 3.x document (JSON or YAML), or `-` for stdin |

## Options

| Flag               | Description                                                                                                                                                                                                                                              |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-f`, `--format`   | `convert`: `postman`, `bruno`, `insomnia`, `hoppscotch`, `rest-client`, `thunder-client`, `yaak`, `hurl`, `kreya`, `curl`, `all` (default: `postman`). `preview`: `swagger-ui`, `redoc`, `scalar`, `stoplight`, `rapidoc`, `all` (default: `swagger-ui`) |
| `-o`, `--out`      | Output directory (`convert` default: `./api-collections`; `preview` default: `./api-docs`); `--output` is also accepted                                                                                                                                  |
| `--name <name>`    | Collection name override (default: `info.title`)                                                                                                                                                                                                         |
| `--base-url <url>` | `convert` only — override the server URL for all requests                                                                                                                                                                                                |

## Global flags

| Flag              | Description   |
| ----------------- | ------------- |
| `-h`, `--help`    | Print help    |
| `-V`, `--version` | Print version |

`api formats` lists every supported format name (client and preview).

## Exit codes

| Code | Meaning                                            |
| ---- | -------------------------------------------------- |
| `0`  | Conversion succeeded                               |
| `1`  | Spec could not be parsed or converted (see stderr) |
| `2`  | CLI usage error                                    |

## Examples

```sh
# Convert a local YAML spec to a Postman collection
api convert ./openapi.yaml

# Emit all client formats into ./dist
api convert ./openapi.json --format all --out ./dist

# Convert from stdin
cat openapi.yaml | api convert - --format bruno --out ./dist

# Bruno collection named after the API, pointed at staging
api convert openapi.yaml --format bruno --name acme --base-url https://staging.acme.dev/v1

# Generate browsable API docs (Swagger UI, ReDoc, Scalar, Stoplight, RapiDoc)
api preview openapi.yaml --format all --out ./docs
```
