import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import RootLayout from '../layout';

describe('RootLayout', () => {
  it('renders children inside the document body', () => {
    render(
      <RootLayout>
        <p>page content</p>
      </RootLayout>
    );
    expect(screen.getByText('page content')).toBeInTheDocument();
  });

  it('renders the navbar and footer', () => {
    render(
      <RootLayout>
        <p>content</p>
      </RootLayout>
    );
    expect(screen.getAllByText('API').length).toBeGreaterThan(0);
    expect(
      screen.getByText(/OpenAPI to API client collections/)
    ).toBeInTheDocument();
  });
});
