import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Navbar from '../Navbar';

const links = [
  { label: 'Features', href: '/#features' },
  { label: 'Docs', href: 'https://example.com/docs' },
];

const groups = [
  {
    label: 'Guides',
    items: [
      { label: 'Install', href: '/docs/getting-started/installation/' },
      { label: 'Formats', href: '/docs/api-reference/formats/' },
    ],
  },
];

describe('Navbar', () => {
  it('renders name, links and the GitHub button', () => {
    render(
      <Navbar
        name="api"
        logoSrc="/logo.svg"
        links={links}
        githubUrl="https://github.com/x"
      />
    );
    expect(screen.getByText('api')).toBeInTheDocument();
    expect(document.querySelector('img')).toHaveAttribute('src', '/logo.svg');
    expect(screen.getByRole('link', { name: 'Features' })).toHaveAttribute(
      'href',
      '/#features'
    );
    expect(screen.getByRole('link', { name: 'Docs' })).toHaveAttribute(
      'href',
      'https://example.com/docs'
    );
    expect(screen.getAllByRole('link', { name: /GitHub/ })[0]).toHaveAttribute(
      'href',
      'https://github.com/x'
    );
  });

  it('toggles a dropdown group open and closed', () => {
    render(
      <Navbar
        name="api"
        links={links}
        groups={groups}
        githubUrl="https://github.com/x"
      />
    );
    const toggle = screen.getByRole('button', { name: /Guides/ });
    expect(toggle).toHaveAttribute('aria-expanded', 'false');
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByRole('link', { name: 'Install' })).toBeInTheDocument();
    fireEvent.click(toggle);
    expect(
      screen.queryByRole('link', { name: 'Install' })
    ).not.toBeInTheDocument();
  });

  it('closes the dropdown after navigating', () => {
    render(
      <Navbar
        name="api"
        links={links}
        groups={groups}
        githubUrl="https://github.com/x"
      />
    );
    fireEvent.click(screen.getByRole('button', { name: /Guides/ }));
    fireEvent.click(screen.getByRole('link', { name: 'Formats' }));
    expect(
      screen.queryByRole('link', { name: 'Formats' })
    ).not.toBeInTheDocument();
  });
});
