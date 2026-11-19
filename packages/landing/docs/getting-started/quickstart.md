# Quickstart

Prefer not to install anything? The landing page includes an in-browser
converter that runs the same Rust core compiled to WebAssembly — paste or drop
an OpenAPI spec, preview it in Swagger UI, and download the client files.

Convert an OpenAPI spec into a Postman collection:

```sh
api convert openapi.yaml --format postman --out ./collections
```

This writes `collections/postman/<name>.postman_collection.json`. Import that
file into Postman (Import → File) and every request is ready to send.

## Other formats

```sh
api convert openapi.yaml --format bruno       # Bruno collection directory
api convert openapi.yaml --format insomnia    # Insomnia v4 export
api convert openapi.yaml --format hoppscotch  # Hoppscotch collection
api convert openapi.yaml --format rest-client # .http file for the REST Client VS Code extension
api convert openapi.yaml --format thunder-client # Thunder Client (VS Code) collection
api convert openapi.yaml --format yaak        # Yaak workspace (YAML files)
api convert openapi.yaml --format hurl        # .hurl files for the hurl CLI
api convert openapi.yaml --format kreya       # Kreya project directory
api convert openapi.yaml --format curl        # runnable bash script of curl commands
```

See [Output formats](../api-reference/formats.md) for exactly what each format
produces.

## Naming the collection

By default the collection is named after `info.title` in the spec. Override it:

```sh
api convert openapi.yaml --format postman --name "Acme API"
```

## Pointing requests at a different server

Servers are taken from the spec's `servers` list. To force a single base URL —
for example a staging gateway — pass `--base-url`:

```sh
api convert openapi.yaml --format postman --base-url https://staging.example.com/v1
```

## All formats at once

Emit every supported format in one run:

```sh
api convert openapi.yaml --format all --out ./collections
```

See the [CLI reference](../api-reference/cli.md) for the full flag list.
