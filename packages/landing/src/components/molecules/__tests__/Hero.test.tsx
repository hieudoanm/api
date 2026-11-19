import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Hero from '../Hero';

const base = {
  title: 'Convert OpenAPI specs',
  description: 'Into client collections.',
  primary: { label: 'Convert', href: '/app/' },
  stats: [
    { value: '10', label: 'formats' },
    { value: '1', label: 'binary' },
  ],
};

describe('Hero', () => {
  it('renders title, description, CTA and stats', () => {
    render(<Hero {...base} />);
    expect(screen.getByText('Convert OpenAPI specs')).toBeInTheDocument();
    expect(screen.getByText('Into client collections.')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Convert' })).toHaveAttribute(
      'href',
      '/app'
    );
    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('formats')).toBeInTheDocument();
  });

  it('renders children and the secondary link', () => {
    render(
      <Hero {...base} secondary={{ label: 'Docs', href: '/docs/' }}>
        <div>terminal</div>
      </Hero>
    );
    expect(screen.getByText('terminal')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Docs' })).toBeInTheDocument();
  });
});
