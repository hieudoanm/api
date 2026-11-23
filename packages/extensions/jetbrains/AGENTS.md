# AGENTS.md — packages/extensions/jetbrains

JetBrains (IntelliJ Platform) plugin: **api → Convert/Preview OpenAPI Spec**.
Kotlin, shells out to the `api` CLI — modeled on `mars/extensions/jetbrains`.

## Commands

```bash
./gradlew runIde        # sandbox IDE with the plugin installed
./gradlew buildPlugin   # → build/distributions/api-jetbrains-<version>.zip
./gradlew test          # JUnit4 tests
./gradlew koverVerify   # 80% line/branch gate on non-excluded classes
```

## Layout

- `src/main/kotlin/com/api/idea/ApiCli.kt` — pure helpers (format lists,
  `isSpecFileName`, `convertArgs`/`previewArgs`, `outputDirFor`). Keep it
  IDE-free so `ApiCliTest` covers it.
- `src/main/kotlin/com/api/idea/actions/ApiActions.kt` — notifications,
  `runCli` (ProcessBuilder off the EDT), spec picking (current editor file or
  file chooser), format picker (`Messages.showDialog`), and the three actions.
- `ApiSettings*.kt` — `api` CLI path persisted to `api.xml`
  (Settings → Tools → api).
- `plugin.xml` — `com.api.idea`; `ToolsMenu` popup group `api`.

## Conventions

- The plugin requires the `api` CLI on PATH (or configured path); it does not
  embed `api-core`. Client formats write `api-collections/<format>/…`,
  previews write `api-docs/<format>/…` — matching the CLI defaults.
- Kover excludes `com.api.idea.ApiSettings*` and `com.api.idea.actions.*`
  (IDE-bound); keep new pure logic in `ApiCli.kt` or another IDE-free class
  with tests to stay above the 80% gate.
- Gradle wrapper is committed; CI caches `~/.gradle` keyed on
  `**/*.gradle*` + wrapper properties.
