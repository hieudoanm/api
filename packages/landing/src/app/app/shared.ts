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
