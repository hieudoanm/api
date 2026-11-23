import type { Metadata } from 'next';
import type { FC } from 'react';
import Link from 'next/link';
import { PiDownloadSimple, PiTerminal } from 'react-icons/pi';

import SpecsBrowser from './specs-browser';
import { SPECS } from './specs';

export const metadata: Metadata = {
  title: 'API Specs',
  description:
    'OpenAPI 3.x specifications for popular public APIs — ready to convert into API client collections or preview as documentation.',
  alternates: { canonical: '/app/apis-specs/' },
};

const ApisPage: FC = () => (
  <main className="mx-auto w-full max-w-6xl px-5 py-16">
    <div className="mb-10 text-center">
      <p className="text-primary text-sm font-semibold tracking-wide uppercase">
        Sample specs
      </p>
      <h1 className="text-base-content m-0 mt-2 text-3xl font-bold tracking-tight text-balance md:text-4xl">
        Try the converter on real APIs.
      </h1>
      <p className="text-base-content/60 mx-auto mt-3 max-w-2xl text-lg">
        Sixty-four OpenAPI 3.x specifications for popular public APIs. Download
        a spec, then feed it to the{' '}
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

    <SpecsBrowser specs={SPECS} />

    <div className="border-base-content/10 bg-base-100/60 mt-10 flex flex-col items-center gap-3 rounded-2xl border p-6 text-center sm:flex-row sm:text-left">
      <div className="bg-primary/10 text-primary shrink-0 rounded-xl p-3">
        <PiTerminal className="h-6 w-6" />
      </div>
      <div className="flex-1">
        <h2 className="text-base-content m-0 text-lg font-bold">
          Prefer the CLI?
        </h2>
        <p className="text-base-content/60 mt-1 text-sm">
          Convert any of these specs locally with{' '}
          <code>api convert spec.yaml --format all</code> — or try it right here
          in the browser.
        </p>
      </div>
      <div className="flex gap-2">
        <Link href="/app/converter/" className="btn btn-primary btn-sm">
          <PiDownloadSimple className="h-4 w-4" />
          Open converter
        </Link>
        <Link href="/app/previewer/" className="btn btn-outline btn-sm">
          Preview docs
        </Link>
      </div>
    </div>
  </main>
);

export default ApisPage;
