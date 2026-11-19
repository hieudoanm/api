# OpenAPI support

api accepts OpenAPI **3.x** documents in JSON or YAML. Swagger / OpenAPI 2.0
specs are rejected with an `UnsupportedVersion` error — convert them first with
a tool like [swagger2openapi](https://www.npmjs.com/package/swagger2openapi).

## What is mapped

| OpenAPI concept                    | api collection                                       |
| ---------------------------------- | ---------------------------------------------------- |
| `info.title` / `info.version`      | collection name and version                          |
| `servers[].url`                    | request base URL (with `servers` variables resolved) |
| `paths` + `operations`             | requests, grouped by first `tags` entry into folders |
| `parameters` (path, query, header) | request parameters with types and required flags     |
| `requestBody`                      | body — JSON, form-urlencoded, multipart or raw text  |
| `securitySchemes` + `security`     | auth — Bearer, Basic, API key, OAuth2                |
| `operation.description`, `summary` | request descriptions                                 |

Operations without tags land in a `Default` folder. Path-level parameters are
merged into each operation. Cookie parameters are currently ignored because the
target clients do not share a common cookie-parameter model.

## Auth handling

Security requirements are read from `security` on the operation, falling back to
the document-level `security`. Supported `securitySchemes` types:

- `http` / `bearer` → Bearer token auth
- `http` / `basic` → Basic auth
- `apiKey` (header or query) → API key auth
- `oauth2` / `openIdConnect` → OAuth2 bearer token

Credentials are emitted as `{{placeholders}}` so each client prompts for the
real secret on first use; replace them in the client or via its environment
variables before sending requests.
