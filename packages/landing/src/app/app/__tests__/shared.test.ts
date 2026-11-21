import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  errorMessage,
  fetchSpec,
  formatBytes,
  isSpecFile,
  slugify,
  specParam,
} from '../shared';

describe('errorMessage', () => {
  it('returns Error.message for Errors', () => {
    expect(errorMessage(new Error('boom'))).toBe('boom');
  });

  it('stringifies non-Error values', () => {
    expect(errorMessage('plain')).toBe('plain');
    expect(errorMessage(42)).toBe('42');
  });
});

describe('formatBytes', () => {
  it('formats bytes, KB and MB', () => {
    expect(formatBytes('abc')).toBe('3 B');
    expect(formatBytes('x'.repeat(2048))).toBe('2.0 KB');
    expect(formatBytes('x'.repeat(2 * 1024 * 1024))).toBe('2.0 MB');
  });
});

describe('slugify', () => {
  it('slugifies names and falls back to openapi', () => {
    expect(slugify('My API v2!')).toBe('my-api-v2');
    expect(slugify('!!!')).toBe('openapi');
  });
});

describe('isSpecFile', () => {
  it('accepts yaml/yml/json filenames', () => {
    expect(isSpecFile('petstore.yaml')).toBe(true);
    expect(isSpecFile('api.openai.com.yaml')).toBe(true);
    expect(isSpecFile('spec.yml')).toBe(true);
    expect(isSpecFile('spec.json')).toBe(true);
  });

  it('rejects other values', () => {
    expect(isSpecFile(null)).toBe(false);
    expect(isSpecFile('')).toBe(false);
    expect(isSpecFile('../etc/passwd')).toBe(false);
    expect(isSpecFile('a/b.yaml')).toBe(false);
    expect(isSpecFile('spec.txt')).toBe(false);
  });
});

describe('specParam', () => {
  afterEach(() => {
    window.history.pushState({}, '', '/');
  });

  it('reads the spec query parameter', () => {
    window.history.pushState({}, '', '/app/converter/?spec=petstore.yaml');
    expect(specParam()).toBe('petstore.yaml');
  });

  it('returns null when absent', () => {
    window.history.pushState({}, '', '/app/converter/');
    expect(specParam()).toBeNull();
  });
});

describe('fetchSpec', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('fetches the spec text', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(new Response('openapi: 3.0.0', { status: 200 }))
      )
    );
    await expect(fetchSpec('petstore.yaml')).resolves.toBe('openapi: 3.0.0');
    expect(fetch).toHaveBeenCalledWith('/apis/petstore.yaml');
  });

  it('throws on non-ok responses', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.resolve(new Response('nope', { status: 404 })))
    );
    await expect(fetchSpec('missing.yaml')).rejects.toThrow(
      'failed to load spec missing.yaml (HTTP 404)'
    );
  });
});
