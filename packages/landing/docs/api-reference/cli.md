# CLI reference

```text
api convert <spec> [options]
```

## Arguments

| Argument | Description                                                      |
| -------- | ---------------------------------------------------------------- |
| `<spec>` | Path to an OpenAPI 3.x document (JSON or YAML), or `-` for stdin |

## Options

| Flag               | Description                                                                                                                                               |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-f`, `--format`   | Output format: `postman`, `bruno`, `insomnia`, `hoppscotch`, `rest-client`, `thunder-client`, `yaak`, `hurl`, `kreya`, `curl`, `all` (default: `postman`) |
| `-o`, `--out`      | Output directory (default: `./api-collections`); `--output` is also accepted                                                                              |
| `--name <name>`    | Collection name override (default: `info.title`)                                                                                                          |
| `--base-url <url>` | Override the server URL for all requests                                                                                                                  |

## Global flags

| Flag              | Description   |
| ----------------- | ------------- |
| `-h`, `--help`    | Print help    |
| `-V`, `--version` | Print version |

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

# Emit all formats into ./dist
api convert ./openapi.json --format all --out ./dist

# Convert from stdin
cat openapi.yaml | api convert - --format bruno --out ./dist

# Bruno collection named after the API, pointed at staging
api convert openapi.yaml --format bruno --name acme --base-url https://staging.acme.dev/v1
```
