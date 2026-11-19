'use client';

import { useEffect, useMemo, useRef, useState } from 'react';
import type { ChangeEvent, DragEvent, FC } from 'react';
import { strToU8, zipSync } from 'fflate';
import init, { convert, convert_all, openapi_info } from '@api/wasm';
import {
  LuCheck,
  LuCopy,
  LuDownload,
  LuEye,
  LuFileJson,
  LuPlay,
  LuSearch,
  LuTrash2,
  LuUpload,
} from 'react-icons/lu';
import SwaggerPreview from './swagger-preview';

type OutputFormat =
  | 'postman'
  | 'bruno'
  | 'insomnia'
  | 'hoppscotch'
  | 'rest-client'
  | 'thunder-client'
  | 'yaak'
  | 'hurl'
  | 'kreya'
  | 'curl'
  | 'all';

const FORMATS: { value: OutputFormat; label: string }[] = [
  { value: 'postman', label: 'Postman' },
  { value: 'bruno', label: 'Bruno' },
  { value: 'insomnia', label: 'Insomnia' },
  { value: 'hoppscotch', label: 'Hoppscotch' },
  { value: 'rest-client', label: 'REST Client (.http)' },
  { value: 'thunder-client', label: 'Thunder Client' },
  { value: 'yaak', label: 'Yaak' },
  { value: 'hurl', label: 'Hurl' },
  { value: 'kreya', label: 'Kreya' },
  { value: 'curl', label: 'curl (.sh)' },
  { value: 'all', label: 'All formats' },
];

interface WasmFile {
  format: string;
  path: string;
  content: string;
}

interface SpecInfo {
  name: string;
  description: string;
  version: string;
  servers: string[];
  folders: { name: string; requests: number }[];
  requests: number;
}

let wasmReady: Promise<unknown> | null = null;
const ensureWasm = (): Promise<unknown> => {
  if (!wasmReady) wasmReady = init();
  return wasmReady;
};

const errorMessage = (error: unknown): string => {
  if (error instanceof Error) return error.message;
  return String(error);
};

const formatBytes = (value: string): string => {
  const bytes = new TextEncoder().encode(value).length;
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

const slugify = (value: string): string => {
  const slug = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return slug || 'openapi';
};

const downloadBlob = (blob: Blob, filename: string): void => {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 1000);
};

const ConverterApp: FC = () => {
  const [source, setSource] = useState('');
  const [format, setFormat] = useState<OutputFormat>('postman');
  const [name, setName] = useState('');
  const [baseUrl, setBaseUrl] = useState('');
  const [wasmState, setWasmState] = useState<'loading' | 'ready' | 'error'>(
    'loading'
  );
  const [wasmError, setWasmError] = useState('');
  const [info, setInfo] = useState<SpecInfo | null>(null);
  const [files, setFiles] = useState<WasmFile[] | null>(null);
  const [selected, setSelected] = useState(0);
  const [busy, setBusy] = useState<'inspect' | 'convert' | null>(null);
  const [error, setError] = useState('');
  const [dragging, setDragging] = useState(false);
  const [copied, setCopied] = useState(false);
  const [showPreview, setShowPreview] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    let cancelled = false;
    ensureWasm()
      .then(() => {
        if (!cancelled) setWasmState('ready');
      })
      .catch((err: unknown) => {
        if (cancelled) return;
        setWasmState('error');
        setWasmError(errorMessage(err));
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const ready = wasmState === 'ready' && !busy;
  const selectedFile = useMemo(
    () => files?.[selected] ?? null,
    [files, selected]
  );
  const collectionName = name.trim() || info?.name || 'openapi';

  const inspect = async (): Promise<void> => {
    if (!source.trim()) {
      setError('Paste or drop an OpenAPI JSON/YAML document first.');
      return;
    }
    setBusy('inspect');
    setError('');
    try {
      const parsed = JSON.parse(openapi_info(source)) as SpecInfo;
      setInfo(parsed);
      setFiles(null);
      setSelected(0);
    } catch (err) {
      setError(errorMessage(err));
    } finally {
      setBusy(null);
    }
  };

  const runConvert = async (): Promise<void> => {
    if (!source.trim()) {
      setError('Paste or drop an OpenAPI JSON/YAML document first.');
      return;
    }
    setBusy('convert');
    setError('');
    try {
      const payload =
        format === 'all'
          ? convert_all(source, name, baseUrl)
          : convert(source, format, name, baseUrl);
      const parsed = JSON.parse(payload) as { files?: WasmFile[] };
      const produced = parsed.files ?? [];
      if (produced.length === 0) {
        throw new Error('converter produced no files');
      }
      setFiles(produced);
      setSelected(0);
      try {
        setInfo(JSON.parse(openapi_info(source)) as SpecInfo);
      } catch {
        setInfo(null);
      }
    } catch (err) {
      setError(errorMessage(err));
      setFiles(null);
    } finally {
      setBusy(null);
    }
  };

  const loadFile = async (file: File): Promise<void> => {
    setError('');
    setInfo(null);
    setFiles(null);
    setSelected(0);
    setSource(await file.text());
  };

  const onFileInput = (event: ChangeEvent<HTMLInputElement>): void => {
    const file = event.target.files?.[0];
    if (file) void loadFile(file);
    event.target.value = '';
  };

  const onDrop = (event: DragEvent<HTMLDivElement>): void => {
    event.preventDefault();
    setDragging(false);
    const file = event.dataTransfer.files?.[0];
    if (file) void loadFile(file);
  };

  const downloadFile = (file: WasmFile): void => {
    const type = file.path.endsWith('.json')
      ? 'application/json'
      : file.path.endsWith('.http')
        ? 'text/plain'
        : 'text/plain';
    downloadBlob(
      new Blob([file.content], { type }),
      file.path.split('/').pop() ?? file.path
    );
  };

  const downloadAll = (): void => {
    if (!files || files.length === 0) return;
    if (files.length === 1) {
      downloadFile(files[0]);
      return;
    }
    const zipped: Record<string, Uint8Array> = {};
    for (const file of files) {
      zipped[`${file.format}/${file.path}`] = strToU8(file.content);
    }
    const archive = zipSync(zipped, { level: 6 });
    downloadBlob(
      new Blob([archive], { type: 'application/zip' }),
      `${slugify(collectionName)}-api-clients.zip`
    );
  };

  const copyContent = async (content: string): Promise<void> => {
    try {
      await navigator.clipboard.writeText(content);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1500);
    } catch (err) {
      setError(errorMessage(err));
    }
  };

  const copySelected = async (): Promise<void> => {
    if (selectedFile) await copyContent(selectedFile.content);
  };

  const clear = (): void => {
    setSource('');
    setInfo(null);
    setFiles(null);
    setSelected(0);
    setError('');
    setCopied(false);
    setShowPreview(false);
  };

  return (
    <main className="mx-auto w-full max-w-6xl px-5 py-12">
      <div className="mb-8">
        <p className="text-primary text-sm font-semibold tracking-wide uppercase">
          Browser converter
        </p>
        <h1 className="text-base-content m-0 mt-2 text-3xl font-bold tracking-tight text-balance md:text-4xl">
          Convert OpenAPI in the browser.
        </h1>
        <p className="text-base-content/60 mt-3 max-w-2xl text-lg">
          The same <code>api-core</code> Rust engine that powers the CLI runs
          here as WebAssembly. Paste a spec or drop a file — nothing is
          uploaded.
        </p>
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]">
        <section className="space-y-4">
          <div
            onDragOver={(event) => {
              event.preventDefault();
              setDragging(true);
            }}
            onDragLeave={() => setDragging(false)}
            onDrop={onDrop}
            className={`border-base-content/10 bg-base-200/40 rounded-2xl border-2 border-dashed p-4 transition-colors ${dragging ? 'border-primary bg-primary/5' : ''}`}>
            <div className="mb-3 flex flex-wrap items-center gap-2">
              <button
                type="button"
                className="btn btn-outline btn-sm"
                onClick={() => fileInputRef.current?.click()}>
                <LuUpload className="h-4 w-4" />
                Drop or choose file
              </button>
              <input
                ref={fileInputRef}
                type="file"
                accept=".json,.yaml,.yml,.txt"
                className="hidden"
                onChange={onFileInput}
              />
              <button
                type="button"
                className="btn btn-ghost btn-sm"
                onClick={clear}
                disabled={!source && !files}>
                <LuTrash2 className="h-4 w-4" />
                Clear
              </button>
              <span className="text-base-content/50 ml-auto text-xs">
                OpenAPI 3.x JSON or YAML
              </span>
            </div>
            <textarea
              value={source}
              onChange={(event) => {
                setSource(event.target.value);
                setInfo(null);
                setFiles(null);
                setSelected(0);
              }}
              spellCheck={false}
              placeholder="Paste an OpenAPI document here, or drop openapi.yaml / openapi.json onto this panel…"
              className="border-base-content/10 bg-base-100 text-base-content focus:border-primary/60 h-80 w-full resize-y rounded-xl border p-4 font-mono text-sm leading-relaxed outline-none"
            />
          </div>

          <div className="border-base-content/10 bg-base-100 rounded-2xl border p-5">
            <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
              <label className="block">
                <span className="text-base-content/70 mb-1 block text-sm font-semibold">
                  Output format
                </span>
                <select
                  value={format}
                  onChange={(event) =>
                    setFormat(event.target.value as OutputFormat)
                  }
                  className="select select-bordered w-full">
                  {FORMATS.map((item) => (
                    <option key={item.value} value={item.value}>
                      {item.label}
                    </option>
                  ))}
                </select>
              </label>
              <label className="block">
                <span className="text-base-content/70 mb-1 block text-sm font-semibold">
                  Collection name
                </span>
                <input
                  value={name}
                  onChange={(event) => setName(event.target.value)}
                  placeholder={info?.name ?? 'info.title'}
                  className="input input-bordered w-full"
                />
              </label>
              <label className="block md:col-span-2">
                <span className="text-base-content/70 mb-1 block text-sm font-semibold">
                  Base URL override
                </span>
                <input
                  value={baseUrl}
                  onChange={(event) => setBaseUrl(event.target.value)}
                  placeholder={info?.servers[0] ?? 'https://api.example.com'}
                  className="input input-bordered w-full"
                />
              </label>
            </div>

            <div className="mt-5 flex flex-wrap items-center gap-3">
              <button
                type="button"
                className="btn btn-outline"
                onClick={() => void inspect()}
                disabled={!ready || !source.trim()}>
                <LuSearch className="h-4 w-4" />
                {busy === 'inspect' ? 'Inspecting…' : 'Inspect spec'}
              </button>
              <button
                type="button"
                className="btn btn-primary"
                onClick={() => void runConvert()}
                disabled={!ready || !source.trim()}>
                <LuPlay className="h-4 w-4" />
                {busy === 'convert' ? 'Converting…' : 'Convert'}
              </button>
              <button
                type="button"
                className="btn btn-outline"
                onClick={() => setShowPreview((value) => !value)}
                disabled={!source.trim()}>
                <LuEye className="h-4 w-4" />
                {showPreview ? 'Hide preview' : 'Preview'}
              </button>
              {wasmState === 'loading' && (
                <span className="text-base-content/50 text-sm">
                  Loading WASM converter…
                </span>
              )}
              {wasmState === 'ready' && (
                <span className="text-success inline-flex items-center gap-1 text-sm font-semibold">
                  <LuCheck className="h-4 w-4" /> WASM ready
                </span>
              )}
            </div>
            {wasmState === 'error' && (
              <div className="alert alert-error mt-4 text-sm">
                Failed to load the WebAssembly converter: {wasmError}
              </div>
            )}
            {error && (
              <div className="alert alert-error mt-4 text-sm">{error}</div>
            )}
          </div>

          {info && (
            <div className="border-base-content/10 bg-base-100 rounded-2xl border p-5">
              <div className="flex flex-wrap items-start justify-between gap-4">
                <div>
                  <h2 className="text-base-content m-0 text-xl font-bold">
                    {info.name}
                  </h2>
                  {info.description && (
                    <p className="text-base-content/60 mt-1 text-sm">
                      {info.description}
                    </p>
                  )}
                </div>
                <div className="text-base-content/60 flex flex-wrap gap-4 text-sm">
                  <span>
                    <strong>{info.requests}</strong> requests
                  </span>
                  <span>
                    <strong>{info.folders.length}</strong> folders
                  </span>
                  <span>v{info.version}</span>
                </div>
              </div>
              <div className="mt-4 flex flex-wrap gap-2">
                {info.servers.slice(0, 3).map((server) => (
                  <code
                    key={server}
                    className="bg-base-200 text-base-content/70 rounded-lg px-2 py-1 text-xs">
                    {server}
                  </code>
                ))}
              </div>
            </div>
          )}
        </section>

        <aside className="space-y-4">
          <div className="border-base-content/10 bg-base-100 rounded-2xl border p-5">
            <div className="mb-4 flex items-center justify-between gap-3">
              <h2 className="text-base-content m-0 text-lg font-bold">
                Output files
              </h2>
              <button
                type="button"
                className="btn btn-primary btn-sm"
                onClick={downloadAll}
                disabled={!files || files.length === 0}>
                <LuDownload className="h-4 w-4" />
                Download {files && files.length > 1 ? 'ZIP' : ''}
              </button>
            </div>

            {!files || files.length === 0 ? (
              <div className="border-base-content/10 text-base-content/50 rounded-xl border border-dashed p-6 text-center text-sm">
                Converted files will appear here.
              </div>
            ) : (
              <ul className="space-y-2">
                {files.map((file, index) => (
                  <li
                    key={`${file.format}-${file.path}-${index}`}
                    className={`rounded-xl border p-3 transition-colors ${
                      index === selected
                        ? 'border-primary bg-primary/5'
                        : 'border-base-content/10 hover:border-primary/40'
                    }`}>
                    <button
                      type="button"
                      className="w-full text-left"
                      onClick={() => setSelected(index)}>
                      <div className="flex items-center gap-2">
                        <LuFileJson className="text-primary h-4 w-4 shrink-0" />
                        <span className="text-base-content truncate font-mono text-xs font-semibold">
                          {file.path}
                        </span>
                      </div>
                      <div className="text-base-content/50 mt-1 flex items-center gap-2 text-xs">
                        <span className="badge badge-sm badge-outline">
                          {file.format}
                        </span>
                        <span>{formatBytes(file.content)}</span>
                      </div>
                    </button>
                    <div className="mt-2 flex gap-2">
                      <button
                        type="button"
                        className="btn btn-ghost btn-xs"
                        onClick={() => downloadFile(file)}>
                        <LuDownload className="h-3.5 w-3.5" /> File
                      </button>
                      <button
                        type="button"
                        className="btn btn-ghost btn-xs"
                        onClick={() => void copyContent(file.content)}>
                        <LuCopy className="h-3.5 w-3.5" />{' '}
                        {copied && index === selected ? 'Copied' : 'Copy'}
                      </button>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>

          {selectedFile && (
            <div className="border-base-content/10 bg-base-100 overflow-hidden rounded-2xl border">
              <div className="border-base-content/10 bg-base-200/60 flex items-center justify-between gap-3 border-b px-4 py-3">
                <span className="text-base-content truncate font-mono text-xs font-semibold">
                  {selectedFile.path}
                </span>
                <button
                  type="button"
                  className="btn btn-ghost btn-xs"
                  onClick={() => void copySelected()}>
                  <LuCopy className="h-3.5 w-3.5" />
                  {copied ? 'Copied' : 'Copy content'}
                </button>
              </div>
              <pre className="text-base-content/80 max-h-[28rem] overflow-auto p-4 font-mono text-xs leading-relaxed">
                {selectedFile.content}
              </pre>
            </div>
          )}
        </aside>
      </div>

      {showPreview && (
        <section className="mt-8">
          <div className="mb-3 flex flex-wrap items-end justify-between gap-3">
            <div>
              <h2 className="text-base-content m-0 text-xl font-bold">
                Swagger UI preview
              </h2>
              <p className="text-base-content/60 mt-1 text-sm">
                Renders the pasted or dropped OpenAPI document locally.
              </p>
            </div>
          </div>
          <SwaggerPreview source={source} />
        </section>
      )}
    </main>
  );
};

export default ConverterApp;
