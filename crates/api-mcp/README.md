# api-mcp

MCP (Model Context Protocol) server for api — exposes OpenAPI→collection
conversion as tools over stdio (newline-delimited JSON-RPC 2.0,
protocol `2024-11-05`).

## Tools

| Tool              | Description                                                                                                                                                                                     |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `list_formats`    | List supported output formats (postman, bruno, insomnia, hoppscotch, rest-client, thunder-client, yaak, hurl, kreya, curl).                                                                     |
| `openapi_info`    | Parse an OpenAPI 3.x doc (`source` text or `path`) and return a summary: name, version, servers, folders, request counts.                                                                       |
| `convert_openapi` | Convert a spec into collection files; returns `{format, path, content}` entries. `formats` takes a comma-separated list or `all` (default `postman`); optional `name` and `base_url` overrides. |

## Usage

Run through the `api` binary:

```sh
api mcp
```

Or pipe JSON-RPC frames directly:

```sh
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | api mcp
```

See `AGENTS.md` for conventions.
