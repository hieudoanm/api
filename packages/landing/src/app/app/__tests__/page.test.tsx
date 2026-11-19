import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import AppPage, { metadata } from '../page';

vi.mock('@api/wasm', () => ({
  default: vi.fn(() => Promise.resolve()),
  init: vi.fn(() => Promise.resolve()),
  convert: vi.fn(() => '{"files":[]}'),
  convert_all: vi.fn(() => '{"files":[]}'),
  openapi_info: vi.fn(() => '{}'),
}));

describe('AppPage', () => {
  it('has converter metadata', () => {
    expect(metadata.title).toBe('Converter');
    expect(metadata.description).toContain('Postman');
  });

  it('renders the converter', () => {
    render(<AppPage />);
    expect(
      screen.getByRole('heading', { name: 'Convert OpenAPI in the browser.' })
    ).toBeInTheDocument();
  });
});
