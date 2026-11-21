import init from '@api/wasm';

export interface WasmFile {
  format: string;
  path: string;
  content: string;
}

export interface SpecInfo {
  name: string;
  description: string;
  version: string;
  servers: string[];
  folders: { name: string; requests: number }[];
  requests: number;
}

let wasmReady: Promise<unknown> | null = null;
export const ensureWasm = (): Promise<unknown> => {
  if (!wasmReady) wasmReady = init();
  return wasmReady;
};

export const errorMessage = (error: unknown): string => {
  if (error instanceof Error) return error.message;
  return String(error);
};

export const formatBytes = (value: string): string => {
  const bytes = new TextEncoder().encode(value).length;
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

export const slugify = (value: string): string => {
  const slug = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return slug || 'openapi';
};

export const downloadBlob = (blob: Blob, filename: string): void => {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 1000);
};

// Must match basePath in next.config.ts (GitHub Pages project site).
const BASE_PATH = process.env.NODE_ENV === 'production' ? '/api' : '';

const SPEC_FILE_PATTERN = /^[\w.-]+\.(yaml|yml|json)$/;

export const isSpecFile = (value: string | null): value is string =>
  value !== null && SPEC_FILE_PATTERN.test(value);

export const specParam = (): string | null => {
  if (typeof window === 'undefined') return null;
  return new URLSearchParams(window.location.search).get('spec');
};

export const fetchSpec = async (file: string): Promise<string> => {
  const response = await fetch(`${BASE_PATH}/apis/${file}`);
  if (!response.ok) {
    throw new Error(`failed to load spec ${file} (HTTP ${response.status})`);
  }
  return response.text();
};
