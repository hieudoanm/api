import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import PublicApisPage, { metadata } from '../page';

describe('PublicApisPage', () => {
  it('has metadata', () => {
    expect(metadata.title).toBe('Public APIs');
    expect(metadata.description).toContain('public APIs');
  });

  it('renders the markdown directory as tables', () => {
    const { container } = render(<PublicApisPage />);
    expect(
      screen.getByRole('heading', { name: 'Public APIs.' })
    ).toBeInTheDocument();
    expect(screen.getByText('OpenWeatherMap')).toBeInTheDocument();
    expect(screen.getByText('Crossref Metadata Search')).toBeInTheDocument();
    // 52 categories from the directory + the extra specs section
    expect(container.querySelectorAll('table').length).toBeGreaterThanOrEqual(
      50
    );
    const specLink = screen
      .getAllByRole('link', { name: /openapi/i })
      .find((l) => l.getAttribute('href')?.includes('api.chess.com.yaml'));
    expect(specLink).toHaveAttribute('href', '/apis/api.chess.com.yaml');
  });
});
