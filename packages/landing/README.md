# api landing page

The landing page and documentation site for api — a Next.js (App Router)
static-export site with Tailwind CSS v4 + DaisyUI, deployed to GitHub Pages at
<https://hieudoanm.github.io/api>.

## Structure

- `src/app/` — Next.js pages: `page.tsx` (landing), `app/` (browser WASM
  converter + Swagger UI preview), `docs/[[...slug]]/page.tsx` (docs site),
  `api/search-index/route.ts` (docs search index), `not-found.tsx`,
  `icon.svg`/`favicon.ico` conventions
- `@api/wasm` — workspace dependency backed by `crates/api-wasm`; the browser
  converter imports it and CI builds `pkg/` before `pnpm run build`
- `src/app/globals.css` — Tailwind v4 import + DaisyUI `api` theme (indigo
  primary) + docs prose tokens
- `src/components/molecules/` — shared sections: `Navbar`, `Hero`, `Features`,
  `CTA`, `Footer`, `DocsSearch`
- `src/lib/docs.ts` — Markdown → HTML pipeline (`marked`): doc loading,
  relative-link rewriting, heading anchors, search text
- `docs/` — the documentation source (Markdown), rendered at build time to
  `/docs/*` routes
- `public/` — static assets served under `/api/` (`logo.svg`, `og.png`,
  `favicon.*`, `robots.txt`, `humans.txt`, `llms.txt`); brand files are synced
  from the repo-root `assets/branding/` by `assets/sync-assets.sh` — never edit
  them here

## Develop

```bash
cd packages/landing
pnpm install
pnpm run dev      # http://localhost:3000/api/
pnpm run build    # static export → out/
pnpm run lint && pnpm run typecheck && pnpm run test
```

`basePath`/`assetPrefix` are `/api` in `next.config.ts` (GitHub Pages project
site). `BASE_PATH` in `src/lib/docs.ts` and `INDEX_URL` in
`src/components/molecules/DocsSearch.tsx` must match.

## Deploy

CI runs `pnpm install --frozen-lockfile && pnpm run build` and uploads
`packages/landing/out` to GitHub Pages (see `.github/workflows/ci.yaml`,
`landing-*` jobs).
