# Collections & formats

Internally, api normalizes every spec into a single `ApiCollection` model —
folders, requests, parameters, bodies and auth — then emits that model in the
target client's native format. Adding a new client means writing one emitter;
the parser never changes.

## The intermediate model

```text
ApiCollection
├── name, description, version
├── servers[]            # candidate base URLs
└── folders[]            # one per OpenAPI tag
    └── requests[]       # method, path, params, body, auth
```

Every emitter receives the same model plus `ConvertOptions` (`collection_name`,
`base_url`) and returns `ConversionFile`s — a path and its contents. Most
formats are a single JSON file; Bruno, Yaak, Hurl and Kreya emit directory
trees, REST Client is a single `.http` file, and curl is a shell script.

## Choosing a format

- **Postman** — the lingua franca; best when teammates already live in Postman.
- **Bruno** — plain-text `.bru` files, git-friendly, offline-first.
- **Insomnia** — Insomnia's v4 JSON export format.
- **Hoppscotch** — Hoppscotch's collection JSON.
- **REST Client** — `.http` files for the REST Client extension in VS Code /
  JetBrains — great for committing request collections to the repo.
- **Thunder Client** — collection JSON for the Thunder Client VS Code extension.
- **Yaak** — YAML files in Yaak's git-sync format; open the folder as a
  workspace.
- **Hurl** — one `.hurl` file per request; run with `hurl --test` or commit to
  git.
- **Kreya** — a `.krproj` project directory for the Kreya desktop client.
- **curl** — an executable bash script of `curl` commands; zero dependencies.

Format quirks and file names are documented in
[Output formats](../api-reference/formats.md).
