import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import CTA from '../CTA';

const base = {
  title: 'Ready to convert?',
  description: 'Point api at your spec.',
  primary: { label: 'Get started', href: '/docs/' },
};

describe('CTA', () => {
  it('renders title, description and the primary link', () => {
    render(<CTA {...base} />);
    expect(screen.getByText('Ready to convert?')).toBeInTheDocument();
    expect(screen.getByText('Point api at your spec.')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Get started' })).toHaveAttribute(
      'href',
      '/docs'
    );
  });

  it('renders an external primary link with target _blank', () => {
    render(
      <CTA
        {...base}
        primary={{
          label: 'GitHub',
          href: 'https://github.com/x',
          external: true,
        }}
      />
    );
    const link = screen.getByRole('link', { name: 'GitHub' });
    expect(link).toHaveAttribute('target', '_blank');
    expect(link).toHaveAttribute('rel', 'noreferrer');
  });

  it('renders an anchor link for hash hrefs', () => {
    render(<CTA {...base} primary={{ label: 'Jump', href: '#top' }} />);
    expect(screen.getByRole('link', { name: 'Jump' })).toHaveAttribute(
      'href',
      '#top'
    );
  });

  it('renders the secondary link when provided', () => {
    render(
      <CTA
        {...base}
        secondary={{ label: 'Docs', href: '/docs/', external: false }}
      />
    );
    expect(screen.getByRole('link', { name: 'Docs' })).toBeInTheDocument();
  });

  it('omits the secondary button when not provided', () => {
    render(<CTA {...base} />);
    expect(
      screen.queryByRole('link', { name: 'Docs' })
    ).not.toBeInTheDocument();
  });
});
