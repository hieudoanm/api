import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AppPage, { metadata } from '../page';

describe('AppPage', () => {
  it('has hub metadata', () => {
    expect(metadata.title).toBe('Apps');
    expect(metadata.description).toContain('OpenAPI');
  });

  it('links to the converter and previewer', () => {
    render(<AppPage />);
    expect(
      screen.getByRole('heading', { name: 'OpenAPI tools in the browser.' })
    ).toBeInTheDocument();
    expect(
      screen.getByRole('link', { name: /open converter/i })
    ).toHaveAttribute('href', '/app/converter');
    expect(
      screen.getByRole('link', { name: /open previewer/i })
    ).toHaveAttribute('href', '/app/previewer');
  });
});
