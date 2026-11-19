import type { Metadata } from 'next';
import type { FC } from 'react';
import Link from 'next/link';
import { LuArrowRight, LuFileJson, LuEye } from 'react-icons/lu';

export const metadata: Metadata = {
  title: 'Apps',
  description:
    'Convert OpenAPI 3.x specs into API client collections or preview API documentation — all in your browser.',
  alternates: { canonical: '/app/' },
};

const AppPage: FC = () => (
  <main className="mx-auto w-full max-w-5xl px-5 py-16">
    <div className="mb-10 text-center">
      <p className="text-primary text-sm font-semibold tracking-wide uppercase">
        Browser apps
      </p>
      <h1 className="text-base-content m-0 mt-2 text-3xl font-bold tracking-tight text-balance md:text-4xl">
        OpenAPI tools in the browser.
      </h1>
      <p className="text-base-content/60 mx-auto mt-3 max-w-2xl text-lg">
        The same <code>api-core</code> Rust engine that powers the CLI runs here
        as WebAssembly. Nothing is uploaded.
      </p>
    </div>

    <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
      <Link
        href="/app/converter/"
        className="border-base-content/10 bg-base-100 hover:border-primary/50 group rounded-2xl border p-6 transition-colors">
        <div className="bg-primary/10 text-primary mb-4 inline-flex rounded-xl p-3">
          <LuFileJson className="h-6 w-6" />
        </div>
        <h2 className="text-base-content m-0 text-xl font-bold">Converter</h2>
        <p className="text-base-content/60 mt-2 text-sm">
          Turn an OpenAPI spec into ready-to-import collections for Postman,
          Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl,
          Kreya and curl.
        </p>
        <span className="text-primary mt-4 inline-flex items-center gap-1 text-sm font-semibold">
          Open converter
          <LuArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
        </span>
      </Link>

      <Link
        href="/app/previewer/"
        className="border-base-content/10 bg-base-100 hover:border-primary/50 group rounded-2xl border p-6 transition-colors">
        <div className="bg-primary/10 text-primary mb-4 inline-flex rounded-xl p-3">
          <LuEye className="h-6 w-6" />
        </div>
        <h2 className="text-base-content m-0 text-xl font-bold">Previewer</h2>
        <p className="text-base-content/60 mt-2 text-sm">
          Render your spec with Swagger UI, ReDoc, Scalar, Stoplight Elements or
          RapiDoc and download the standalone HTML page.
        </p>
        <span className="text-primary mt-4 inline-flex items-center gap-1 text-sm font-semibold">
          Open previewer
          <LuArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
        </span>
      </Link>
    </div>
  </main>
);

export default AppPage;
