# api for JetBrains IDEs

IntelliJ Platform plugin for [api](https://github.com/hieudoanm/api) — convert
OpenAPI 3.x JSON/YAML specs into API client collections and HTML docs previews.
Kotlin, built with the `org.jetbrains.intellij` Gradle plugin.

## Features

- **Convert OpenAPI Spec** — `api → Convert OpenAPI Spec` converts the spec
  backing the current editor (or a file picked via dialog) into one of the
  client formats: postman, bruno, insomnia, hoppscotch, rest-client,
  thunder-client, yaak, hurl, kreya, curl — or `all`. Output lands in
  `api-collections/<format>/…` next to the spec file.
- **Preview OpenAPI Spec** — same flow for the docs-UI formats: swagger-ui,
  redoc, scalar, stoplight, rapidoc — or `all`. Output lands in
  `api-docs/<format>/…`.
- **List api Formats** — shows the client formats supported by the installed
  `api` CLI.
- **Configurable CLI path** — Settings → Tools → api.

## Requirements

- A JetBrains IDE 2024.3+ (IntelliJ IDEA Community, Ultimate, PyCharm, etc.).
- The `api` CLI on `PATH`, or set the path in Settings → Tools → api.
  Install it with `scripts/install.sh` from the
  [api repository](https://github.com/hieudoanm/api).

## Build & run

```bash
cd packages/extensions/jetbrains
./gradlew runIde   # launches a sandbox IDE with the plugin installed
```

To build a distributable plugin zip:

```bash
./gradlew buildPlugin
```

Tests and the coverage gate (80% line/branch on the pure-Kotlin helpers):

```bash
./gradlew test koverVerify
```

The plugin targets the 2024.3 IntelliJ Platform (`IC`); adjust the platform
type/version in `build.gradle.kts` to match your IDE.

## Layout

```text
src/main/
├── resources/META-INF/plugin.xml     # services, configurable, actions
└── kotlin/com/api/idea/
    ├── ApiSettings*.kt               # CLI path setting (Settings → Tools → api)
    ├── ApiCli.kt                     # pure helpers: formats, arg building
    └── actions/ApiActions.kt         # convert/preview/list actions, runCli
```
