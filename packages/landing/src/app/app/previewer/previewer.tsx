'use client';

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type { ChangeEvent, DragEvent, FC } from 'react';
import { convert, convert_all, openapi_info } from '@api/wasm';
import { log } from '@/lib/log';
import {
  LuCheck,
  LuCopy,
  LuDownload,
  LuExternalLink,
  LuFileJson,
  LuPlay,
  LuSearch,
  LuTrash2,
  LuUpload,
} from 'react-icons/lu';
import { downloadBlob, ensureWasm, errorMessage, formatBytes } from '../shared';
import type { SpecInfo, WasmFile } from '../shared';

type PreviewFormat =
  'swagger-ui' | 'redoc' | 'scalar' | 'stoplight' | 'rapidoc' | 'all';

const PREVIEW_FORMATS = new Set([
  'swagger-ui',
  'redoc',
  'scalar',
  'stoplight',
  'rapidoc',
]);

const FORMATS: { value: PreviewFormat; label: string }[] = [
  { value: 'swagger-ui', label: 'Swagger UI' },
  { value: 'redoc', label: 'ReDoc' },
  { value: 'scalar', label: 'Scalar' },
  { value: 'stoplight', label: 'Stoplight Elements' },
  { value: 'rapidoc', label: 'RapiDoc' },
  { value: 'all', label: 'All previews' },
];

const PreviewerApp: FC = () => {
  const [source, setSource] = useState('');
  const [format, setFormat] = useState<PreviewFormat>('swagger-ui');
  const [name, setName] = useState('');
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
  const fileInputRef = useRef<HTMLInputElement>(null);
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    const onError = (event: ErrorEvent): void => {
      log.error('[previewer]', event.message, event.filename, event.lineno);
    };
    const onRejection = (event: PromiseRejectionEvent): void => {
      log.error('[previewer]', 'unhandled rejection', event.reason);
    };
    window.addEventListener('error', onError);
    window.addEventListener('unhandledrejection', onRejection);
    return () => {
      window.removeEventListener('error', onError);
      window.removeEventListener('unhandledrejection', onRejection);
    };
  }, []);

  const selectedFile = useMemo(
    () => files?.[selected] ?? null,
    [files, selected]
  );

  const onIframeLoad = useCallback((): void => {
    const frame = iframeRef.current?.contentWindow;
    if (!frame) return;
    const label = selectedFile?.path ?? 'preview';
    frame.addEventListener('error', (event: Event) => {
      const errorEvent = event as ErrorEvent;
      log.error(`[preview:${label}]`, errorEvent.message);
    });
    frame.addEventListener('unhandledrejection', (event: Event) => {
      const rejection = event as PromiseRejectionEvent;
      log.error(`[preview:${label}]`, 'unhandled rejection', rejection.reason);
    });
  }, [selectedFile]);

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
        log.error('[previewer] wasm init failed', err);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const ready = wasmState === 'ready' && !busy;
  const previewUrl = useMemo(() => {
    if (!selectedFile || !selectedFile.path.endsWith('.html')) return null;
    return URL.createObjectURL(
      new Blob([selectedFile.content], { type: 'text/html' })
    );
  }, [selectedFile]);
  useEffect(() => {
    return () => {
      if (previewUrl) URL.revokeObjectURL(previewUrl);
    };
  }, [previewUrl]);

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
      log.error('[previewer] inspect failed', err);
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
          ? convert_all(source, name, '')
          : convert(source, format, name, '');
      const parsed = JSON.parse(payload) as { files?: WasmFile[] };
      const produced = (parsed.files ?? []).filter((file) =>
        PREVIEW_FORMATS.has(file.format)
      );
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
      log.error('[previewer] convert failed', err);
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
    downloadBlob(
      new Blob([file.content], { type: 'text/html' }),
      file.path.split('/').pop() ?? file.path
    );
  };

  const copyContent = async (content: string): Promise<void> => {
    try {
      await navigator.clipboard.writeText(content);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1500);
    } catch (err) {
      setError(errorMessage(err));
      log.error('[previewer] clipboard copy failed', err);
    }
  };

  const clear = (): void => {
    setSource('');
    setInfo(null);
    setFiles(null);
    setSelected(0);
    setError('');
    setCopied(false);
  };

  return (
    <main className="mx-auto w-full max-w-6xl px-5 py-12">
      <div className="mb-8">
        <p className="text-primary text-sm font-semibold tracking-wide uppercase">
          Browser previewer
        </p>
        <h1 className="text-base-content m-0 mt-2 text-3xl font-bold tracking-tight text-balance md:text-4xl">
          Preview API docs in the browser.
        </h1>
        <p className="text-base-content/60 mt-3 max-w-2xl text-lg">
          Render your OpenAPI spec with Swagger UI, ReDoc, Scalar, Stoplight
          Elements or RapiDoc — generated locally by the same Rust engine as the
          CLI.
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
                  Preview format
                </span>
                <select
                  value={format}
                  onChange={(event) =>
                    setFormat(event.target.value as PreviewFormat)
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
                {busy === 'convert' ? 'Generating…' : 'Generate preview'}
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
            </div>
          )}

          {previewUrl && selectedFile && (
            <div className="border-base-content/10 bg-base-100 overflow-hidden rounded-2xl border">
              <div className="border-base-content/10 bg-base-200/60 flex flex-wrap items-center justify-between gap-3 border-b px-4 py-3">
                <span className="text-base-content truncate font-mono text-xs font-semibold">
                  {selectedFile.path}
                </span>
                <div className="flex gap-2">
                  <a
                    href={previewUrl}
                    target="_blank"
                    rel="noreferrer"
                    className="btn btn-ghost btn-xs">
                    <LuExternalLink className="h-3.5 w-3.5" /> Open
                  </a>
                  <button
                    type="button"
                    className="btn btn-ghost btn-xs"
                    onClick={() => downloadFile(selectedFile)}>
                    <LuDownload className="h-3.5 w-3.5" /> Download
                  </button>
                </div>
              </div>
              <iframe
                ref={iframeRef}
                onLoad={onIframeLoad}
                title={`${selectedFile.path} preview`}
                src={previewUrl}
                sandbox="allow-scripts allow-same-origin allow-popups"
                className="bg-base-100 h-[70vh] w-full"
              />
            </div>
          )}
        </section>

        <aside className="space-y-4">
          <div className="border-base-content/10 bg-base-100 rounded-2xl border p-5">
            <h2 className="text-base-content m-0 mb-4 text-lg font-bold">
              Preview files
            </h2>
            {!files || files.length === 0 ? (
              <div className="border-base-content/10 text-base-content/50 rounded-xl border border-dashed p-6 text-center text-sm">
                Generated HTML previews will appear here.
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
        </aside>
      </div>
    </main>
  );
};

export default PreviewerApp;
