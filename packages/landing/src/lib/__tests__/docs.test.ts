import { describe, expect, it } from 'vitest';
import { DOC_SECTIONS, getDocPages, sectionLabel } from '@/lib/docs';

describe('getDocPages', () => {
  it('returns the docs overview and section pages', () => {
    const pages = getDocPages();
    expect(pages.length).toBeGreaterThan(0);
    expect(pages.some((p) => p.slug === '')).toBe(true);
    for (const page of pages) {
      expect(page.title.length).toBeGreaterThan(0);
      expect(page.html.length).toBeGreaterThan(0);
    }
  });

  it('produces unique slugs', () => {
    const slugs = getDocPages().map((p) => p.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it('assigns every page a known section or the overview', () => {
    const ids = new Set(DOC_SECTIONS.map((s) => s.id));
    for (const page of getDocPages()) {
      expect(page.section === '' || ids.has(page.section)).toBe(true);
    }
  });

  it('adds anchor ids to headings and lists them in the toc', () => {
    const page = getDocPages().find((p) => p.toc.length > 0);
    expect(page).toBeDefined();
    expect(page!.html).toMatch(/<h[2-6] id="/);
    for (const h of page!.toc) {
      expect(h.id.length).toBeGreaterThan(0);
      expect(h.text.length).toBeGreaterThan(0);
      expect(h.level).toBeGreaterThanOrEqual(2);
    }
  });

  it('rewrites relative .md links to docs routes', () => {
    const page = getDocPages().find((p) => p.html.includes('/docs/'));
    expect(page).toBeDefined();
    expect(page!.html).not.toMatch(/href="[^"]*\.md/);
  });

  it('extracts plain text for search', () => {
    const page = getDocPages()[0];
    expect(page.text.length).toBeGreaterThan(0);
    expect(page.text).not.toMatch(/<[^>]+>/);
  });
});

describe('sectionLabel', () => {
  it('returns the label for a known section and the id otherwise', () => {
    expect(sectionLabel(DOC_SECTIONS[0].id)).toBe(DOC_SECTIONS[0].label);
    expect(sectionLabel('unknown')).toBe('unknown');
  });
});
