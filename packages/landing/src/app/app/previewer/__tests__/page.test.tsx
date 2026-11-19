import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import PreviewerPage, { metadata } from '../page';

vi.mock('@api/wasm', () => ({
  default: vi.fn(() => Promise.resolve()),
  init: vi.fn(() => Promise.resolve()),
  convert: vi.fn(() => '{"files":[]}'),
  convert_all: vi.fn(() => '{"files":[]}'),
  openapi_info: vi.fn(() => '{}'),
}));

describe('PreviewerPage', () => {
  it('has metadata and renders the previewer', () => {
    expect(metadata.title).toBe('Previewer');
    expect(metadata.description).toContain('Swagger UI');
    expect(metadata.alternates?.canonical).toBe('/app/previewer/');
    render(<PreviewerPage />);
    expect(
      screen.getByRole('heading', {
        name: 'Preview API docs in the browser.',
      })
    ).toBeInTheDocument();
  });
});
