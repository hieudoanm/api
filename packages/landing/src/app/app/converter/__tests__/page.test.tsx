import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import ConverterPage, { metadata } from '../page';

vi.mock('@api/wasm', () => ({
  default: vi.fn(() => Promise.resolve()),
  init: vi.fn(() => Promise.resolve()),
  convert: vi.fn(() => '{"files":[]}'),
  convert_all: vi.fn(() => '{"files":[]}'),
  openapi_info: vi.fn(() => '{}'),
}));

describe('ConverterPage', () => {
  it('has metadata and renders the converter', () => {
    expect(metadata.title).toBe('Converter');
    expect(metadata.description).toContain('Postman');
    expect(metadata.alternates?.canonical).toBe('/app/converter/');
    render(<ConverterPage />);
    expect(
      screen.getByRole('heading', { name: 'Convert OpenAPI in the browser.' })
    ).toBeInTheDocument();
  });
});
