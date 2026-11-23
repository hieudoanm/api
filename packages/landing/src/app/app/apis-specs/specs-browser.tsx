'use client';

import { useMemo, useState } from 'react';
import type { FC } from 'react';
import Link from 'next/link';
import {
  PiCaretLeft,
  PiCaretRight,
  PiDownloadSimple,
  PiEye,
  PiFileText,
  PiPlay,
  PiMagnifyingGlass,
  PiX,
} from 'react-icons/pi';

import { specHref } from './specs';
import type { ApiSpec } from './specs';

interface SpecsBrowserProps {
  specs: ApiSpec[];
}

const PAGE_SIZE = 24;

const SpecsBrowser: FC<SpecsBrowserProps> = ({ specs }) => {
  const [query, setQuery] = useState('');
  const [category, setCategory] = useState('All');
  const [page, setPage] = useState(1);

  const categories = useMemo(
    () => ['All', ...new Set(specs.map((spec) => spec.category))].sort(),
    [specs]
  );

  const filtered = useMemo(() => {
    const needle = query.trim().toLowerCase();
    return specs.filter((spec) => {
      if (category !== 'All' && spec.category !== category) return false;
      if (!needle) return true;
      return [spec.name, spec.tagline, spec.category, spec.file]
        .join(' ')
        .toLowerCase()
        .includes(needle);
    });
  }, [specs, query, category]);

  const totalPages = Math.max(1, Math.ceil(filtered.length / PAGE_SIZE));
  const currentPage = Math.min(page, totalPages);
  const paged = filtered.slice(
    (currentPage - 1) * PAGE_SIZE,
    currentPage * PAGE_SIZE
  );

  return (
    <>
      <div className="mb-6 flex flex-col gap-3">
        <div className="relative">
          <PiMagnifyingGlass className="text-base-content/40 pointer-events-none absolute top-1/2 left-3.5 h-4 w-4 -translate-y-1/2" />
          <input
            type="search"
            value={query}
            onChange={(event) => {
              setQuery(event.target.value);
              setPage(1);
            }}
            placeholder="Search APIs — name, description, category or file…"
            aria-label="Search API specs"
            className="input input-bordered w-full pl-10"
          />
          {query && (
            <button
              type="button"
              onClick={() => setQuery('')}
              aria-label="Clear search"
              className="text-base-content/40 hover:text-base-content absolute top-1/2 right-3 -translate-y-1/2">
              <PiX className="h-4 w-4" />
            </button>
          )}
        </div>
        <div className="flex flex-wrap items-center gap-2">
          {categories.map((name) => (
            <button
              key={name}
              type="button"
              onClick={() => {
                setCategory(name);
                setPage(1);
              }}
              aria-pressed={category === name}
              className={`badge badge-lg cursor-pointer border transition-colors ${
                category === name
                  ? 'badge-primary border-primary text-primary-content'
                  : 'badge-outline border-base-content/20 hover:border-primary/60'
              }`}>
              {name}
            </button>
          ))}
        </div>
        <p className="text-base-content/40 text-sm" aria-live="polite">
          {filtered.length === specs.length
            ? `${specs.length} specs`
            : `Showing ${filtered.length} of ${specs.length} specs`}
        </p>
      </div>

      {filtered.length === 0 ? (
        <div className="border-base-content/10 rounded-2xl border p-10 text-center">
          <p className="text-base-content/60">
            No specs match{' '}
            {category !== 'All' ? `the “${category}” category` : 'your search'}
            {query.trim() ? ` for “${query.trim()}”` : ''}.
          </p>
          <button
            type="button"
            className="btn btn-outline btn-sm mt-4"
            onClick={() => {
              setQuery('');
              setCategory('All');
              setPage(1);
            }}>
            Clear filters
          </button>
        </div>
      ) : (
        <>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
            {paged.map((spec) => (
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
                    href={specHref(spec.file)}
                    prefetch={false}
                    className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
                    <PiFileText className="h-4 w-4" />
                    Spec
                  </Link>
                  <Link
                    href={specHref(spec.file)}
                    prefetch={false}
                    download={spec.file}
                    className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
                    <PiDownloadSimple className="h-4 w-4" />
                    Download
                  </Link>
                  <Link
                    href={`/app/converter/?spec=${spec.file}`}
                    className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
                    <PiPlay className="h-4 w-4" />
                    Convert
                  </Link>
                  <Link
                    href={`/app/previewer/?spec=${spec.file}`}
                    className="text-primary inline-flex items-center gap-1.5 whitespace-nowrap">
                    <PiEye className="h-4 w-4" />
                    Preview
                  </Link>
                </div>
              </div>
            ))}
          </div>
          {totalPages > 1 && (
            <nav
              aria-label="Pagination"
              className="mt-8 flex items-center justify-center gap-2">
              <button
                type="button"
                className="btn btn-outline btn-sm border-base-content/20 hover:border-primary/60"
                disabled={currentPage === 1}
                onClick={() => setPage(currentPage - 1)}>
                <PiCaretLeft className="h-4 w-4" />
                Prev
              </button>
              <div className="join">
                {Array.from(
                  { length: totalPages },
                  (_, index) => index + 1
                ).map((number) => (
                  <button
                    key={number}
                    type="button"
                    aria-current={number === currentPage ? 'page' : undefined}
                    className={`join-item btn btn-sm ${
                      number === currentPage
                        ? 'btn-primary'
                        : 'btn-outline border-base-content/20 hover:border-primary/60'
                    }`}
                    onClick={() => setPage(number)}>
                    {number}
                  </button>
                ))}
              </div>
              <button
                type="button"
                className="btn btn-outline btn-sm border-base-content/20 hover:border-primary/60"
                disabled={currentPage === totalPages}
                onClick={() => setPage(currentPage + 1)}>
                Next
                <PiCaretRight className="h-4 w-4" />
              </button>
            </nav>
          )}
        </>
      )}
    </>
  );
};

export default SpecsBrowser;
