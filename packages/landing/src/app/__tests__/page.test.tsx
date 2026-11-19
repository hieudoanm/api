import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Home from '../page';

describe('Home', () => {
  it('renders the hero, formats and quickstart sections', () => {
    render(<Home />);
    expect(screen.getByRole('heading', { level: 1 })).toBeInTheDocument();
    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('Postman')).toBeInTheDocument();
    expect(screen.getByText('acme.sh')).toBeInTheDocument();
    expect(
      screen.getByRole('link', { name: /Star on GitHub/ })
    ).toBeInTheDocument();
  });
});
