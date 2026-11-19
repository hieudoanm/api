import type { MetadataRoute } from 'next';
import { describe, expect, it } from 'vitest';
import sitemap from '../sitemap';

describe('sitemap', () => {
  it('returns the site root, docs index and every doc page', () => {
    const entries: MetadataRoute.Sitemap = sitemap();
    const urls = entries.map((e) => e.url);
    expect(urls[0]).toBe('https://hieudoanm.github.io/api/');
    expect(urls).toContain('https://hieudoanm.github.io/api/docs/');
    expect(urls.some((u) => u.includes('/docs/getting-started/'))).toBe(true);
    expect(new Set(urls).size).toBe(urls.length);
  });
});
