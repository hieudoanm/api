import { describe, expect, it } from 'vitest';
import { DOC_SECTIONS, sectionLabel } from '@/lib/sections';

describe('DOC_SECTIONS', () => {
  it('lists the four doc sections in order', () => {
    expect(DOC_SECTIONS.map((s) => s.id)).toEqual([
      'getting-started',
      'concepts',
      'api-reference',
      'guides',
    ]);
  });
});

describe('sectionLabel', () => {
  it('returns the label for a known section', () => {
    expect(sectionLabel('getting-started')).toBe('Getting started');
    expect(sectionLabel('api-reference')).toBe('API reference');
  });

  it('returns the id unchanged for an unknown section', () => {
    expect(sectionLabel('unknown')).toBe('unknown');
    expect(sectionLabel('')).toBe('');
  });
});
