import type { Metadata } from 'next';
import type { FC } from 'react';
import Link from 'next/link';
import {
  LuDownload,
  LuEye,
  LuFileText,
  LuPlay,
  LuTerminal,
} from 'react-icons/lu';

export const metadata: Metadata = {
  title: 'API Specs',
  description:
    'OpenAPI 3.x specifications for popular public APIs — ready to convert into API client collections or preview as documentation.',
  alternates: { canonical: '/app/apis/' },
};

// Must match basePath in next.config.ts (GitHub Pages project site).
// Raw hrefs don't get basePath applied automatically.
const BASE_PATH = process.env.NODE_ENV === 'production' ? '/api' : '';

interface ApiSpec {
  file: string;
  name: string;
  tagline: string;
  category: string;
  endpoints: number;
}

const SPECS: ApiSpec[] = [
  {
    file: 'api.chess.com.yaml',
    name: 'Chess.com',
    tagline:
      'Published Data API — players, games, clubs, puzzles and leaderboards.',
    category: 'Gaming',
    endpoints: 30,
  },
  {
    file: 'lichess.org.api.yaml',
    name: 'Lichess',
    tagline:
      'Open-source chess platform — games, puzzles, tournaments, TV and more.',
    category: 'Gaming',
    endpoints: 190,
  },
  {
    file: 'api.openai.com.yaml',
    name: 'OpenAI',
    tagline:
      'Official spec — chat, responses, assistants, audio, realtime and more.',
    category: 'AI',
    endpoints: 388,
  },
  {
    file: 'api.generativelanguage.googleapis.com.yaml',
    name: 'Gemini',
    tagline:
      'Google Gemini API — generateContent, streaming, batch and tuned models.',
    category: 'AI',
    endpoints: 50,
  },
  {
    file: 'api.anthropic.com.yaml',
    name: 'Claude',
    tagline:
      'Anthropic Messages API — messages, batches, files and organizations.',
    category: 'AI',
    endpoints: 14,
  },
  {
    file: 'api.deepseek.com.yaml',
    name: 'DeepSeek',
    tagline: 'OpenAI-compatible chat and legacy completions API.',
    category: 'AI',
    endpoints: 4,
  },
  {
    file: 'api.telegram.org.yaml',
    name: 'Telegram Bot',
    tagline:
      'Telegram Bot API — 135 methods for messages, media, payments and more.',
    category: 'Messaging',
    endpoints: 135,
  },
  {
    file: 'newsapi.org.yaml',
    name: 'NewsAPI',
    tagline:
      'Live news articles — search everything, top headlines and sources.',
    category: 'News',
    endpoints: 3,
  },
  {
    file: 'api.openweathermap.org.yaml',
    name: 'OpenWeatherMap',
    tagline: 'Current weather, forecasts, air pollution and weather stations.',
    category: 'Weather',
    endpoints: 7,
  },
  {
    file: 'api.airvisual.com.v2.yaml',
    name: 'AirVisual',
    tagline: 'IQAir air quality data — cities, stations and rankings.',
    category: 'Weather',
    endpoints: 9,
  },
  {
    file: 'api.frankfurter.dev.v2.yaml',
    name: 'Frankfurter v2',
    tagline: 'Free exchange-rate API — current, historical and provider rates.',
    category: 'Finance',
    endpoints: 9,
  },
  {
    file: 'api.frankfurter.dev.v1.yaml',
    name: 'Frankfurter v1',
    tagline:
      'Legacy Frankfurter endpoints — latest, date ranges and currencies.',
    category: 'Finance',
    endpoints: 5,
  },
  {
    file: 'data.fixer.io.api.yaml',
    name: 'Fixer.io',
    tagline: 'Exchange rates — latest, historical, timeseries and conversion.',
    category: 'Finance',
    endpoints: 6,
  },
  {
    file: 'api.coinranking.com.v2.yaml',
    name: 'Coinranking',
    tagline: 'Crypto market data — coins, exchanges, markets and statistics.',
    category: 'Crypto',
    endpoints: 34,
  },
  {
    file: 'api.football.data.org.yaml',
    name: 'football-data.org',
    tagline:
      'Football/soccer data — competitions, matches, teams and standings.',
    category: 'Sports',
    endpoints: 15,
  },
  {
    file: 'api.crossref.org.yaml',
    name: 'Crossref',
    tagline:
      'Scholarly metadata — works, journals, funders, members and prefixes.',
    category: 'Research',
    endpoints: 18,
  },
];

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
        Sixteen OpenAPI 3.x specifications for popular public APIs. Download a
        spec, then feed it to the{' '}
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

    <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {SPECS.map((spec) => (
        <div
          key={spec.file}
          className="border-base-content/10 bg-base-100 hover:border-primary/50 flex flex-col rounded-2xl border p-5 transition-colors">
          <div className="flex items-start justify-between gap-2">
            <h2 className="text-base-content m-0 text-base font-bold">
              {spec.name}
            </h2>
            <span className="badge badge-outline badge-sm shrink-0">
              {spec.category}
            </span>
          </div>
          <p className="text-base-content/60 mt-2 flex-1 text-sm">
            {spec.tagline}
          </p>
          <p className="text-base-content/40 mt-3 text-xs">
            {spec.endpoints} endpoints
          </p>
          <div className="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-sm font-semibold">
            <Link
              href={`${BASE_PATH}/apis/${spec.file}`}
              prefetch={false}
              className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
              <LuFileText className="h-4 w-4" />
              Spec
            </Link>
            <Link
              href={`${BASE_PATH}/apis/${spec.file}`}
              prefetch={false}
              download={spec.file}
              className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
              <LuDownload className="h-4 w-4" />
              Download
            </Link>
            <Link
              href={`/app/converter/?spec=${spec.file}`}
              className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
              <LuPlay className="h-4 w-4" />
              Convert
            </Link>
            <Link
              href={`/app/previewer/?spec=${spec.file}`}
              className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
              <LuEye className="h-4 w-4" />
              Preview
            </Link>
          </div>
        </div>
      ))}
    </div>

    <div className="border-base-content/10 bg-base-100/60 mt-10 flex flex-col items-center gap-3 rounded-2xl border p-6 text-center sm:flex-row sm:text-left">
      <div className="bg-primary/10 text-primary shrink-0 rounded-xl p-3">
        <LuTerminal className="h-6 w-6" />
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
          <LuDownload className="h-4 w-4" />
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
