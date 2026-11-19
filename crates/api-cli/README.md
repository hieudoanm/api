# api-cli

The `api` command-line binary: converts OpenAPI 3.x specs (JSON or YAML) into
API client collections for Postman, Bruno, Insomnia, Hoppscotch, REST
Client, Thunder Client, Yaak, Hurl, Kreya and curl, and serves the same conversion over MCP.

## Commands

```sh
api convert <input> [options]   # input: file path or `-` for stdin
api formats                     # list supported output formats
api mcp                         # MCP server over stdio (JSON-RPC 2.0)
api --version
```

## convert options

| Flag                       | Description                                          |
| -------------------------- | ---------------------------------------------------- |
| `-o`, `--out` (`--output`) | Output directory (default `./api-collections`)       |
| `-f`, `--format`           | Comma-separated formats or `all` (default `postman`) |
| `--name`                   | Collection name override (default `info.title`)      |
| `--base-url`               | Base URL override for all requests                   |

Output is written under `<out>/<format>/` — e.g.
`api-collections/postman/petstore.postman_collection.json`,
`api-collections/bruno/…`, `api-collections/rest-client/petstore.http`.

## Examples

```sh
api convert openapi.yaml
api convert openapi.yaml -f all -o ./dist
cat openapi.yaml | api convert - -f bruno
api convert petstore.yaml --name "Swagger Petstore" --base-url https://staging.example.com/v1
```

Conversion itself lives in [`api-core`](../api-core); the MCP interface in
[`api-mcp`](../api-mcp). See the crate `AGENTS.md` for conventions.
