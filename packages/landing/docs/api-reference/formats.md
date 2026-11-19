# Output formats

`api convert --format <name>` accepts: `postman`, `bruno`, `insomnia`,
`hoppscotch`, `rest-client`, `thunder-client`, `yaak`, `hurl`, `kreya`, `curl` —
or `all` to emit every format at once.

## Postman

A single `<name>.postman_collection.json` in Postman Collection v2.1 schema —
importable via Postman → Import → File. Folders map to OpenAPI tags, requests
carry their own auth, and `{{variables}}` are used for tokens.

## Bruno

A `bruno/` directory containing `bruno.json`, `collection.bru`,
`environments/development.bru` and one `.bru` file per request, organized into
subdirectories per folder/tag. `.bru` files are plain text and diff cleanly in
git.

## Insomnia

A single `<name>.insomnia.json` in Insomnia's export format (resources array
with `request` / `request_group` / `workspace` resources).

## Hoppscotch

A single `<name>.hoppscotch.json` matching Hoppscotch's collection schema —
importable from the app's collection import screen.

## REST Client

A single `<name>.http` file with one `###`-separated block per request, REST
Client extension syntax — query params, headers and bodies inline, `@variable`
declarations for auth tokens.

## Thunder Client

A single `thunder-collection_<name>.json` in Thunder Client's collection export
format — importable into the VS Code extension via Collection → Import.

## Yaak

A directory of YAML files in Yaak's directory-sync format: `workspace.yaml`,
`environment-development.yaml`, one `folder-*.yaml` per tag and one
`request-*.yaml` per operation. Open the folder as a Yaak workspace (New
Workspace → Open Folder) to use it.

## Hurl

A directory of `.hurl` files — one per request, organized into subdirectories
per folder/tag. Run them with `hurl --test <dir>` or open them in any editor;
they are plain text and diff cleanly in git.

## Kreya

A `<name>/` project directory in Kreya's file-based format: `<name>.krproj`,
`directory.krpref`, `development.krenv` and one `.krop` file per operation (with
a sibling `-request.json`/`.txt` body file when the request has a body). Open
the directory as a Kreya project.

## curl

A single executable `<name>.sh` bash script with one `curl` command per request,
a `BASE_URL` variable at the top, and an HTTP status code printed per call. Run
it directly or lift individual commands into your shell history.

## File names

| Format         | Output                             |
| -------------- | ---------------------------------- |
| postman        | `<name>.postman_collection.json`   |
| insomnia       | `<name>.insomnia.json`             |
| hoppscotch     | `<name>.hoppscotch.json`           |
| rest-client    | `<name>.http`                      |
| thunder-client | `thunder-collection_<name>.json`   |
| curl           | `<name>.sh`                        |
| bruno          | `bruno/` directory                 |
| yaak           | `yaak/` directory of YAML files    |
| hurl           | `hurl/` directory of `.hurl` files |
| kreya          | `<name>/` project directory        |

For single-file formats, `<name>` is `--name` when given, otherwise `info.title`
from the spec sanitized to a file-safe string. Directory formats always write
under `<out>/<format>/` and store the collection name inside their metadata
files.
