# Using api in CI

The most common use of api is regenerating client collections whenever the
OpenAPI spec changes — so the generated files never drift from the API.

## GitHub Actions example

```yaml
name: api collections
on:
  push:
    paths: [openapi.yaml]
jobs:
  convert:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install --git https://github.com/hieudoanm/api --bin api
      - run: api convert openapi.yaml --format all --out ./collections
      - uses: stefanzweifel/git-auto-commit-action@v5
        with:
          commit_message: 'chore: regenerate api collections'
```

## Committing generated collections

Which formats are worth committing depends on the client:

- **REST Client / Bruno / Hurl / Yaak / Kreya** — plain text or YAML/JSON trees,
  review diffs in pull requests.
- **Postman / Insomnia / Hoppscotch / Thunder Client** — single JSON blobs;
  commit them if your team imports from the repo, otherwise publish them as
  build artifacts.

A typical layout:

```text
api/
├── openapi.yaml          # source of truth
└── collections/
    ├── acme.http         # REST Client — committed
    └── acme/             # Bruno — committed
```

Run `api convert` locally before pushing spec changes to catch conversion errors
early — the same command CI runs.
