# AGENTS.md — api landing page

Guidance for AI agents working on the landing page (`packages/landing`). See the
root `AGENTS.md` for repository-wide conventions.

## Overview

Next.js 16 (App Router) static-export site + Tailwind CSS v4 + DaisyUI, deployed
to GitHub Pages at `https://hieudoanm.github.io/api` (project site —
`basePath: "/api"`).

## Layout

- `src/app/page.tsx` — landing page (hero, features, output-format cards,
  quickstart terminals, CTA, footer)
- `src/app/app/` — browser apps hub (`/app/`) linking to:
  - `app/converter/` (`/app/converter/`) — paste/drop OpenAPI specs and download
    converted API client files (10 formats) via the WASM build of `api-core`
  - `app/previewer/` (`/app/previewer/`) — paste/drop OpenAPI specs and
    download/preview standalone HTML docs (Swagger UI, ReDoc, Scalar, Stoplight
    Elements, RapiDoc) via the same WASM build
  - `app/converter.tsx`, `app/previewer/previewer.tsx`, `app/shared.ts` — the
    client components behind those routes
- `@api/wasm` — workspace dependency backed by `crates/api-wasm`; its generated
  `pkg/` bindings are gitignored and built with `make build-wasm` (needs the
  `wasm32-unknown-unknown` Rust target and `wasm-bindgen` CLI)
- `src/app/docs/[[...slug]]/page.tsx` — docs catch-all; renders every Markdown
  file under `docs/` as a static page at `/api/docs/<path>/`
- `src/app/globals.css` — `@import 'tailwindcss'` + official plugins only:
  `@tailwindcss/typography` for docs prose and DaisyUI with a custom `api` theme
  (indigo `primary`, see Styling below)
- `src/lib/docs.ts` — Markdown → HTML pipeline (`marked`), relative-link
  rewriting to `/api/docs/*`, GitHub-style heading ids
- `docs/` — documentation source
- `public/` — static assets served under `/api/`
- `src/app/icon.svg`, `src/app/favicon.ico` — Next metadata file conventions

## Commands

```sh
cd packages/landing
pnpm install
pnpm run dev     # dev server (basePath applies: /api/)
pnpm run build   # static export → out/ (uploaded to Pages by CI)
pnpm run lint && pnpm run typecheck && pnpm run test
pnpm run test:coverage   # vitest + v8 coverage; thresholds 80% (stmts/branch/funcs/lines)

# rebuild the wasm package consumed by /app (from repo root or crates/api-wasm)
pnpm --filter @api/wasm run build:wasm
```

## Conventions

- **Fonts:** Inter (sans) via `next/font` (`--font-inter`); mono stack
  `ui-monospace, 'SF Mono', SFMono-Regular, Menlo, Consolas, monospace`. Do not
  add other typefaces.

- **Keep `output: "export"`** — the site must build to static HTML; no server
  features, no API routes, no `next/image` optimization.
- **`basePath`/`assetPrefix` are `/api`.** `next/link` and metadata get it
  automatically; raw `<a href>` and generated HTML links must include it (see
  `BASE_PATH` in `src/lib/docs.ts`, and the `INDEX_URL` in
  `src/components/molecules/DocsSearch.tsx` → `/api/api/search-index`).
- **DaisyUI + Tailwind v4** — `globals.css` has only `@import 'tailwindcss'`
  plus official plugins; the `api` theme (set via `data-theme` on `<html>`) maps
  `primary` to blue (`#2563eb`). Use DaisyUI components (`btn`, `badge`, `card`)
  and theme tokens (`bg-base-100`, `text-base-content`, `text-primary`,
  `bg-primary/10`) — no custom CSS.
- **Docs are Markdown**, rendered at build time — no MDX, no client JS. Links
  between docs use relative `.md` paths; `src/lib/docs.ts` rewrites them. Links
  escaping the docs tree are pointed at the GitHub repo blob.
- **Pin dependency versions** in `package.json`; CI runs
  `pnpm install --frozen-lockfile` against `pnpm-lock.yaml` — commit lockfile
  changes.
- **Deploy:** CI builds `out/` and uploads via `upload-pages-artifact` +
  `deploy-pages` (see `.github/workflows/ci.yaml`, `landing-*` jobs).
- **Coverage:** keep test coverage at or above 80% for new and changed code —
  `pnpm run test:coverage` enforces 80% on statements, branches, functions and
  lines (vitest v8 thresholds in `vitest.config.ts`).

## Footer

The footer follows the shared contract via
`src/components/molecules/Footer.tsx`: `© {year} {repo-name}` (lowercase repo
name, `new Date().getFullYear()`), a one-line tagline/license descriptor, and a
right-aligned `<nav aria-label="Footer">` with exactly two links — `GitHub`
(repo root) and `README` (`<repo>#readme`), both
`target="_blank" rel="noreferrer"`. Reuse the component on every page that shows
a footer.

## Features & CTA sections

The landing page composes `Features` and `CTA` components (same directory as
`Footer`). Features: centered head (h2 + supporting p), responsive card grid,
card = icon chip + h3 + p. CTA: alt/surface background, centered h2 + p, primary
action button plus at most one ghost button. Keep copy and link targets
per-repo; keep the structure shared.

## Styling

Tailwind CSS only. `globals.css` contains just `@import 'tailwindcss';` plus
official plugins (`@tailwindcss/typography` for docs prose, DaisyUI). No
`@theme`, `@layer`, `@utility`, or custom selectors — style with utilities in
`className`. Inter comes from `next/font` `className` on `<html>`; smooth scroll
uses `scroll-smooth motion-reduce:scroll-auto`. Markdown-rendered HTML gets
utility classes emitted by `src/lib/docs.ts`, never custom CSS. Inline
`style={{}}` is only for genuinely dynamic values.

## Assets

`public/` is **generated output** — never edit brand assets here. The canonical
sources live in `assets/branding/svg/` at the repo root; run
`assets/sync-assets.sh` to copy `logo.svg`, render `og.png` (1200×630) and
rebuild `favicon.ico` into `public/`. `src/app/icon.svg` and
`public/favicon.svg` mirror `logo.svg` — keep them in sync manually.
`assets/svg-to-png.sh` regenerates all `*/png/` outputs after SVG edits.
