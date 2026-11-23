import type { Metadata, NextPage } from 'next';
import Link from 'next/link';
import type { FC } from 'react';
import type { IconType } from 'react-icons';
import Features, { type Feature } from '@/components/molecules/Features';
import Hero from '@/components/molecules/Hero';
import CTA from '@/components/molecules/CTA';
import {
  PiArrowsLeftRight,
  PiBracketsCurly,
  PiCheck,
  PiCloudSlash,
  PiFileCode,
  PiFileJs,
  PiTreeStructure,
  PiKey,
  PiStack,
  PiPackage,
  PiRepeat,
  PiTerminal,
} from 'react-icons/pi';

export const metadata: Metadata = {
  alternates: { canonical: '/' },
};

const GITHUB = 'https://github.com/hieudoanm/api';

type TokKind =
  'prompt' | 'cmd' | 'flag' | 'str' | 'key' | 'url' | 'dim' | 'ok' | 'err';
interface Tok {
  t: string;
  c?: TokKind;
}

const TK: Record<TokKind, string> = {
  prompt: 'text-accent select-none',
  cmd: 'font-semibold text-sky-300',
  flag: 'text-amber-400',
  str: 'text-green-300',
  key: 'text-violet-300',
  url: 'text-cyan-300',
  dim: 'text-slate-400',
  ok: 'text-gray-200',
  err: 'text-red-400',
};

interface TerminalProps {
  lines: Tok[][];
}

const Terminal: FC<TerminalProps> = ({ lines }) => (
  <div className="mockup-code">
    {lines.map((line, i) => {
      const prompt = line[0]?.c === 'prompt' ? line[0].t.trim() : null;
      const tokens = prompt ? line.slice(1) : line;
      return (
        <pre key={i} {...(prompt ? { 'data-prefix': prompt } : {})}>
          <code>
            {tokens.map((tok, j) => (
              <span key={j} className={TK[tok.c ?? 'ok']}>
                {tok.t}
              </span>
            ))}
          </code>
        </pre>
      );
    })}
  </div>
);

const FEATURES: Feature[] = [
  {
    title: 'OpenAPI 3.x in, collections out',
    body: 'JSON or YAML specs are normalized into one intermediate model, then emitted in the target client’s native format. No spec rewriting, no SaaS.',
    icon: PiArrowsLeftRight,
  },
  {
    title: 'Ten client formats',
    body: 'Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — or all of them in a single run with --format all.',
    icon: PiStack,
  },
  {
    title: 'Auth schemes mapped',
    body: 'Bearer, Basic, API key, OAuth2 and OpenID Connect security schemes become each client’s native auth — with {{placeholders}} for secrets.',
    icon: PiKey,
  },
  {
    title: 'Tags become folders',
    body: 'OpenAPI tags organize requests into folders exactly like the spec author intended — no flat walls of endpoints.',
    icon: PiTreeStructure,
  },
  {
    title: 'Bodies & parameters',
    body: 'Path, query and header parameters plus JSON, form, multipart and raw bodies — content types preserved per media type.',
    icon: PiBracketsCurly,
  },
  {
    title: 'One Rust binary',
    body: 'api-core does the conversion, api-cli is a thin clap binary on top. Small, fast, auditable — cargo install and go.',
    icon: PiPackage,
  },
  {
    title: 'CI-friendly',
    body: 'Regenerate collections on every spec change and commit the diff — REST Client and Bruno output are plain text that reviews cleanly.',
    icon: PiRepeat,
  },
  {
    title: 'Offline by design',
    body: 'Your spec never leaves the machine. No accounts, no telemetry, no cloud conversion service — just a file in, files out.',
    icon: PiCloudSlash,
  },
  {
    title: 'Library + CLI',
    body: 'api-core is a normal Rust crate — embed conversion in your own tooling, generators or doc pipelines, not just the CLI.',
    icon: PiTerminal,
  },
];

interface FormatCard {
  name: string;
  file: string;
  body: string;
  icon: IconType;
}

const FORMATS: FormatCard[] = [
  {
    name: 'Postman',
    file: 'acme.postman_collection.json',
    body: 'Collection v2.1 — import via Postman → Import → File.',
    icon: PiFileJs,
  },
  {
    name: 'Bruno',
    file: 'bruno/  (bruno.json + *.bru)',
    body: 'Plain-text .bru files — git-friendly, offline-first.',
    icon: PiTreeStructure,
  },
  {
    name: 'Insomnia',
    file: 'acme.insomnia.json',
    body: 'Insomnia v4 export — workspace, groups and requests.',
    icon: PiFileJs,
  },
  {
    name: 'Hoppscotch',
    file: 'acme.hoppscotch.json',
    body: 'Hoppscotch collection JSON — import from the app.',
    icon: PiFileJs,
  },
  {
    name: 'REST Client',
    file: 'acme.http',
    body: '.http blocks for the REST Client VS Code / JetBrains extension.',
    icon: PiFileCode,
  },
  {
    name: 'Thunder Client',
    file: 'thunder-collection_acme.json',
    body: 'Thunder Client export JSON — import into the VS Code extension.',
    icon: PiFileJs,
  },
  {
    name: 'Yaak',
    file: 'yaak/  (workspace.yaml + *.yaml)',
    body: 'YAML files in Yaak’s directory-sync format — open as a workspace.',
    icon: PiTreeStructure,
  },
  {
    name: 'Hurl',
    file: 'hurl/  (*.hurl per request)',
    body: 'One .hurl file per request — run with hurl --test or commit to git.',
    icon: PiFileCode,
  },
  {
    name: 'Kreya',
    file: 'acme/  (*.krproj project)',
    body: 'Kreya project directory — .krop operations plus body files.',
    icon: PiTreeStructure,
  },
  {
    name: 'curl',
    file: 'acme.sh',
    body: 'Executable bash script of curl commands — zero dependencies.',
    icon: PiTerminal,
  },
];

const Home: NextPage = () => (
  <>
    <main id="top">
      {/* hero */}
      <Hero
        title="Your OpenAPI spec, in every API client your team uses."
        description="api is a Rust CLI that converts OpenAPI 3.x documents into ready-to-import collections for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — folders, parameters, bodies and auth included. One binary, no cloud, no accounts."
        primary={{ label: 'Get started', href: '#quickstart' }}
        secondary={{ label: 'Source on GitHub', href: GITHUB, external: true }}
        stats={[
          { value: '10', label: 'client formats' },
          { value: '1', label: 'Rust binary' },
          { value: '0', label: 'accounts needed' },
        ]}>
        <Terminal
          lines={[
            [
              { t: '$ ', c: 'prompt' },
              { t: 'cargo', c: 'cmd' },
              {
                t: ' install --git https://github.com/hieudoanm/api api-cli',
              },
            ],
            [
              { t: '$ ', c: 'prompt' },
              { t: 'api', c: 'cmd' },
              { t: ' convert openapi.yaml ', c: 'dim' },
              { t: '--format', c: 'flag' },
              { t: ' all ', c: 'dim' },
              { t: '--out', c: 'flag' },
              { t: ' ./collections', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'postman', c: 'key' },
              { t: '   → acme.postman_collection.json', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'bruno', c: 'key' },
              { t: '    → bruno/', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'insomnia', c: 'key' },
              { t: ' → acme.insomnia.json', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'hoppscotch', c: 'key' },
              { t: ' → acme.hoppscotch.json', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'rest-client', c: 'key' },
              { t: ' → acme.http', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'thunder-client', c: 'key' },
              { t: ' → thunder-collection_acme.json', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'yaak', c: 'key' },
              { t: '         → yaak/', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'hurl', c: 'key' },
              { t: '           → hurl/', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'kreya', c: 'key' },
              { t: '         → acme/', c: 'dim' },
            ],
            [
              { t: '✓ ', c: 'ok' },
              { t: 'curl', c: 'key' },
              { t: '           → acme.sh', c: 'dim' },
            ],
            [
              { t: '$ ', c: 'prompt' },
              { t: 'code', c: 'cmd' },
              { t: ' acme.http', c: 'dim' },
              { t: '   # REST Client extension — send away', c: 'dim' },
            ],
          ]}
        />
      </Hero>

      <Features
        title="Spec in. Collections out. Nothing else."
        description="api-core parses the OpenAPI document once into a normalized model — folders, requests, parameters, bodies, auth — and each emitter renders it in a client's native format."
        features={FEATURES}
      />

      {/* formats */}
      <section id="formats" className="bg-gray-50 py-20">
        <div className="mx-auto w-full max-w-6xl px-5">
          <div className="mx-auto mb-12 max-w-2xl text-center">
            <h2 className="m-0 text-3xl leading-tight tracking-normal text-balance">
              Every format your team already imports.
            </h2>
            <p className="mt-3 text-lg text-gray-500">
              One spec, ten destinations — pick the client, or emit them all and
              let each teammate choose.
            </p>
          </div>
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {FORMATS.map((f) => (
              <div
                key={f.name}
                className="card border-base-300 bg-base-100 border p-6">
                <div className="bg-primary/10 text-primary mb-3.5 flex h-11 w-11 items-center justify-center rounded-lg">
                  <f.icon className="h-6 w-6" />
                </div>
                <h3 className="text-base-content m-0 mb-1 text-lg leading-tight text-balance">
                  {f.name}
                </h3>
                <code className="text-primary mb-2 block font-mono text-xs break-all">
                  {f.file}
                </code>
                <p className="text-base-content/60 text-sm">{f.body}</p>
              </div>
            ))}
            <div className="border-primary/30 bg-primary/5 flex flex-col justify-center rounded-2xl border border-dashed p-6 text-center">
              <PiCheck className="text-primary mx-auto mb-2 h-6 w-6" />
              <p className="text-base-content m-0 text-sm font-semibold">
                --format all
              </p>
              <p className="text-base-content/60 mt-1 text-sm">
                Emit every format in one run.
              </p>
            </div>
          </div>
          <p className="mt-4 text-center text-sm text-gray-500">
            File names and schemas per format are documented in{' '}
            <Link href="/docs/api-reference/formats/" className="text-primary">
              Output formats
            </Link>
            .
          </p>
        </div>
      </section>

      {/* quickstart */}
      <section id="quickstart" className="py-20">
        <div className="mx-auto w-full max-w-6xl px-5">
          <div className="mx-auto mb-12 max-w-2xl text-center">
            <h2 className="m-0 text-3xl leading-tight tracking-normal text-balance">
              From spec to sendable request in one command.
            </h2>
            <p className="mt-3 text-lg text-gray-500">
              Prebuilt binaries on the{' '}
              <Link href={`${GITHUB}/releases`} className="text-primary">
                releases page
              </Link>
              , or build from source with Cargo.
            </p>
          </div>
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
            <Terminal
              lines={[
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'git', c: 'cmd' },
                  {
                    t: ' clone https://github.com/hieudoanm/api',
                    c: 'url',
                  },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'cd', c: 'cmd' },
                  { t: ' api', c: 'dim' },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'cargo', c: 'cmd' },
                  { t: ' build ', c: 'dim' },
                  { t: '--release', c: 'flag' },
                  { t: ' -p ', c: 'dim' },
                  { t: 'api-cli', c: 'flag' },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: './target/release/api', c: 'cmd' },
                  { t: ' --version', c: 'dim' },
                ],
                [{ t: 'api 0.0.1', c: 'ok' }],
              ]}
            />
            <Terminal
              lines={[
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'api', c: 'cmd' },
                  { t: ' convert', c: 'dim' },
                  { t: ' petstore.yaml', c: 'str' },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'api', c: 'cmd' },
                  { t: ' convert', c: 'dim' },
                  { t: ' petstore.yaml ', c: 'str' },
                  { t: '--format', c: 'flag' },
                  { t: ' postman ', c: 'dim' },
                  { t: '--name', c: 'flag' },
                  { t: ' "Swagger Petstore"', c: 'str' },
                ],
                [
                  { t: '✓ ', c: 'ok' },
                  { t: 'Wrote ', c: 'dim' },
                  {
                    t: 'swagger-petstore.postman_collection.json',
                    c: 'url',
                  },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'api', c: 'cmd' },
                  { t: ' convert', c: 'dim' },
                  { t: ' petstore.json ', c: 'str' },
                  { t: '--format', c: 'flag' },
                  { t: ' rest-client ', c: 'dim' },
                  { t: '--base-url', c: 'flag' },
                  { t: ' https://petstore3.swagger.io/api/v3', c: 'url' },
                ],
                [
                  { t: '✓ ', c: 'ok' },
                  { t: 'Wrote ', c: 'dim' },
                  { t: 'petstore.http', c: 'url' },
                ],
              ]}
            />
          </div>
          <p className="mt-8 text-center text-sm text-gray-500">
            Full docs at{' '}
            <Link href="/docs/" className="text-primary">
              /docs
            </Link>
            . Specs are read locally — nothing is uploaded anywhere.
          </p>
        </div>
      </section>

      <CTA
        title="Stop hand-building collections."
        description="Point api at your OpenAPI spec and let your team import the result into whichever client they already use."
        primary={{ label: 'Download', href: '/downloads/' }}
        secondary={{ label: 'View Docs', href: '/docs/' }}
      />
    </main>
  </>
);

export default Home;
