import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import ApisPage, { metadata } from '../page';

describe('ApisPage', () => {
  it('has metadata', () => {
    expect(metadata.title).toBe('API Specs');
    expect(metadata.description).toContain('OpenAPI');
  });

  it('renders all forty-eight spec cards with spec, convert and preview links', () => {
    render(<ApisPage />);
    expect(
      screen.getByRole('heading', { name: 'Try the converter on real APIs.' })
    ).toBeInTheDocument();
    const specLinks = screen.getAllByRole('link', { name: /^spec$/i });
    const convertLinks = screen.getAllByRole('link', { name: /convert/i });
    const previewLinks = screen.getAllByRole('link', { name: /preview/i });
    expect(specLinks).toHaveLength(48);
    // 48 cards + intro paragraph link + CTA button
    expect(convertLinks).toHaveLength(50);
    expect(previewLinks).toHaveLength(50);
    expect(specLinks[0]).toHaveAttribute(
      'href',
      '/apis/yaml/api.chess.com.yaml'
    );
    const chessConvert = convertLinks.find((link) =>
      link.getAttribute('href')?.includes('spec=api.chess.com.yaml')
    );
    expect(chessConvert).toHaveAttribute(
      'href',
      '/app/converter?spec=api.chess.com.yaml'
    );
    expect(screen.getByText('OpenAI')).toBeInTheDocument();
    expect(screen.getByText('388 endpoints')).toBeInTheDocument();
    expect(screen.getByText('Supabase')).toBeInTheDocument();
    expect(screen.getByText('Spotify')).toBeInTheDocument();
    expect(screen.getByText('NASA Open APIs')).toBeInTheDocument();
    expect(screen.getByText('Trello')).toBeInTheDocument();
    expect(screen.getByText('SendGrid')).toBeInTheDocument();
  });
});
