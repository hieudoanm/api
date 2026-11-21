import type { Metadata } from 'next';
import type { FC } from 'react';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { marked } from 'marked';
import Link from 'next/link';

export const metadata: Metadata = {
  title: 'Public APIs',
  description:
    'A curated list of free public APIs with ready-made OpenAPI specs for the browser converter and previewer.',
  alternates: { canonical: '/app/public-apis/' },
};

// Must match basePath in next.config.ts (GitHub Pages project site).
// Used in raw HTML hrefs, so basePath is not applied automatically.
const BASE_PATH = process.env.NODE_ENV === 'production' ? '/api' : '';

const DOC_PATH = path.join(process.cwd(), 'src/docs/public-api.md');

const prefixInternalLinks = (html: string): string =>
  html.replace(/href="\/(apis|app)\//g, `href="${BASE_PATH}/$1/`);

const renderMarkdown = (): string => {
  const markdown = readFileSync(DOC_PATH, 'utf8');
  return prefixInternalLinks(
    marked.parse(markdown, { async: false }) as string
  );
};

const PublicApisPage: FC = () => (
  <main className="mx-auto w-full max-w-4xl px-5 py-16">
    <div className="mb-8">
      <p className="text-primary text-sm font-semibold tracking-wide uppercase">
        Directory
      </p>
      <h1 className="text-base-content m-0 mt-2 text-3xl font-bold tracking-tight text-balance md:text-4xl">
        Public APIs.
      </h1>
      <p className="text-base-content/60 mt-3 max-w-2xl text-lg">
        A curated list of free public APIs. Entries with an{' '}
        <strong>OpenAPI</strong> link ship a spec you can feed straight into the{' '}
        <Link href="/app/converter/" className="text-primary font-semibold">
          converter
        </Link>{' '}
        or{' '}
        <Link href="/app/previewer/" className="text-primary font-semibold">
          previewer
        </Link>
        .
      </p>
    </div>

    <article
      className="prose max-w-none"
      dangerouslySetInnerHTML={{ __html: renderMarkdown() }}
    />
  </main>
);

export default PublicApisPage;
