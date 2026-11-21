// Bundle the extension to a single CJS file and copy the wasm binary next to
// it — the extension host has no bundler at runtime.

import { build } from 'esbuild';
import { copyFileSync, mkdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const wasm = path.resolve(
  root,
  '../../../crates/api-wasm/pkg/api_wasm_bg.wasm'
);

// api_wasm.js (wasm-bindgen glue) references import.meta.url, which does not
// exist in the CJS bundle. Point it at the bundle file itself so the default
// init() resolves api_wasm_bg.wasm sitting next to dist/extension.js.
const shimImportMeta = {
  name: 'shim-import-meta',
  setup(b) {
    b.onLoad({ filter: /api-wasm[/\\]pkg[/\\]api_wasm\.js$/ }, async (args) => {
      const contents = await readFileSync(args.path, 'utf8');
      return {
        contents: contents.replaceAll(
          'import.meta.url',
          'require("node:url").pathToFileURL(__filename).href'
        ),
        loader: 'js',
      };
    });
  },
};

mkdirSync(path.join(root, 'dist'), { recursive: true });

await build({
  entryPoints: [path.join(root, 'src/extension.ts')],
  bundle: true,
  platform: 'node',
  format: 'cjs',
  target: 'node22',
  external: ['vscode'],
  plugins: [shimImportMeta],
  outfile: path.join(root, 'dist/extension.js'),
  sourcemap: true,
  logLevel: 'warning',
});

copyFileSync(wasm, path.join(root, 'dist/api_wasm_bg.wasm'));
console.log('built dist/extension.js + dist/api_wasm_bg.wasm');
