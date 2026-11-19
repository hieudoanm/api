# AGENTS.md — assets

Rules for working in `assets/`.

## Sources of truth

- `branding/svg/*.svg` are the canonical logo/OG sources. `branding/png/`
  is generated output — never edit PNGs directly.
- `demo/svg/demo.svg` is the animated terminal demo (SMIL `<animate>`);
  `demo/svg/terminal.svg` is its static final frame. `demo/png/` and
  `demo/gif/` are generated.

## After editing an SVG

1. Run `assets/svg-to-png.sh` (requires `rsvg-convert`) to regenerate the
   sibling `png/` outputs.
2. If the SVG feeds the landing page (`branding/*`), run
   `assets/sync-assets.sh` to update `packages/landing/public/`.
3. `demo.gif` has no in-repo generator — flag to the user that it needs
   regeneration when `demo.svg` changes.

## Content rules

- Terminal/demo text must match real `api` behaviour:
  - Commands: `api convert <input> [-o dir] [-f formats] [--name] [--base-url]`,
    `api formats`, `api --version`. Input is a file path or `-` for stdin —
    there is **no URL fetching**.
  - `-f/--format` takes a comma-separated list (`postman,bruno,…`) or `all`;
    default is `postman`.
  - Output lands in `<out>/<format>/…` (default `./api-collections`); `wrote …`
    diagnostics go to stderr.
- Keep the demo dark-terminal aesthetic: `#0b0b0b` background, `#141414`
  window, `#93c5fd` prompt (blue, matching the `api` theme), `#7dd3a8`
  success, mono font stack.
- `og.svg` is 1280x640 (2:1); `logo.svg` is a 512x512 badge — bold "API"
  wordmark (`#2563eb`, Helvetica/Arial/sans-serif, weight 800) on white. Keep
  the font stack quote-free: ImageMagick's SVG renderer fails on quoted
  family names, so `favicon.ico` is built via `rsvg-convert` → PNG → `magick`
  instead of rendering the SVG directly.
