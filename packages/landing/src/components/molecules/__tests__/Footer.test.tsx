import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Footer from '../Footer';

describe('Footer', () => {
  it('renders name, tagline, year and links', () => {
    render(
      <Footer
        name="api"
        tagline="OpenAPI to collections"
        githubUrl="https://github.com/hieudoanm/api"
      />
    );
    expect(
      screen.getByText(`© ${new Date().getFullYear()} api`)
    ).toBeInTheDocument();
    expect(screen.getByText('OpenAPI to collections')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /GitHub/ })).toHaveAttribute(
      'href',
      'https://github.com/hieudoanm/api'
    );
    expect(screen.getByRole('link', { name: 'README' })).toHaveAttribute(
      'href',
      'https://github.com/hieudoanm/api#readme'
    );
  });
});
