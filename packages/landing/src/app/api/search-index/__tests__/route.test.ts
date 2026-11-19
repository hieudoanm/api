import { describe, expect, it } from 'vitest';
import { GET } from '../route';

describe('search-index GET', () => {
  it('returns a JSON array of doc entries', async () => {
    const response = GET();
    expect(response.status).toBe(200);
    const index = (await response.json()) as {
      slug: string;
      title: string;
      section: string;
      headings: string[];
      text: string;
    }[];
    expect(Array.isArray(index)).toBe(true);
    expect(index.length).toBeGreaterThan(0);
    expect(index.some((e) => e.slug === '')).toBe(true);
    for (const entry of index) {
      expect(entry.title.length).toBeGreaterThan(0);
      expect(entry.text.length).toBeGreaterThan(0);
      expect(Array.isArray(entry.headings)).toBe(true);
    }
  });
});
