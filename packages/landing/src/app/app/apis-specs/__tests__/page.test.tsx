import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import ApisPage, { metadata } from '../page';

describe('ApisPage', () => {
  it('has metadata', () => {
    expect(metadata.title).toBe('API Specs');
    expect(metadata.description).toContain('OpenAPI');
  });

  it('renders the first page of spec cards with spec, convert and preview links', () => {
    render(<ApisPage />);
    expect(
      screen.getByRole('heading', { name: 'Try the converter on real APIs.' })
    ).toBeInTheDocument();
    const specLinks = screen.getAllByRole('link', { name: /^spec$/i });
    const convertLinks = screen.getAllByRole('link', { name: /convert/i });
    const previewLinks = screen.getAllByRole('link', { name: /preview/i });
    // 24 cards on page 1 + intro paragraph link + CTA button
    expect(specLinks).toHaveLength(24);
    expect(convertLinks).toHaveLength(26);
    expect(previewLinks).toHaveLength(26);
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
    expect(screen.getByText('Asana')).toBeInTheDocument();
    // page 2 items are not rendered yet
    expect(screen.queryByText('Supabase')).not.toBeInTheDocument();
  });

  it('filters specs by search query', () => {
    render(<ApisPage />);
    fireEvent.change(screen.getByLabelText('Search API specs'), {
      target: { value: 'spotify' },
    });
    expect(screen.getAllByRole('link', { name: /^spec$/i })).toHaveLength(1);
    expect(screen.getByText('Spotify')).toBeInTheDocument();
    expect(screen.queryByText('OpenAI')).not.toBeInTheDocument();
    expect(screen.getByText(/Showing 1 of 64 specs/)).toBeInTheDocument();
  });

  it('filters specs by category', () => {
    render(<ApisPage />);
    fireEvent.click(screen.getByRole('button', { name: 'Gaming' }));
    const specLinks = screen.getAllByRole('link', { name: /^spec$/i });
    expect(specLinks).toHaveLength(5);
    expect(screen.getByText('Chess.com')).toBeInTheDocument();
    expect(screen.getByText('D&D 5e API')).toBeInTheDocument();
    expect(screen.queryByText('OpenAI')).not.toBeInTheDocument();
  });

  it('paginates through all specs', () => {
    render(<ApisPage />);
    const nav = screen.getByRole('navigation', { name: 'Pagination' });
    expect(nav).toBeInTheDocument();
    // 64 specs / 24 per page → 3 pages (24 + 24 + 16)
    expect(screen.getAllByRole('link', { name: /^spec$/i })).toHaveLength(24);
    fireEvent.click(screen.getByRole('button', { name: '2' }));
    expect(screen.getAllByRole('link', { name: /^spec$/i })).toHaveLength(24);
    expect(screen.getByText('Supabase')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: '3' }));
    expect(screen.getAllByRole('link', { name: /^spec$/i })).toHaveLength(16);
    expect(screen.getByText('Wikimedia')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Next' })).toBeDisabled();
  });

  it('resets to page 1 when filters change', () => {
    render(<ApisPage />);
    fireEvent.click(screen.getByRole('button', { name: '2' }));
    fireEvent.change(screen.getByLabelText('Search API specs'), {
      target: { value: 'spotify' },
    });
    // pagination disappears for a single result
    expect(
      screen.queryByRole('navigation', { name: 'Pagination' })
    ).not.toBeInTheDocument();
    expect(screen.getByText('Spotify')).toBeInTheDocument();
  });

  it('shows an empty state and clears filters', () => {
    render(<ApisPage />);
    fireEvent.change(screen.getByLabelText('Search API specs'), {
      target: { value: 'zzzz-no-match' },
    });
    expect(screen.getByText(/No specs match/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Clear filters' }));
    expect(screen.getAllByRole('link', { name: /^spec$/i })).toHaveLength(24);
  });
});
