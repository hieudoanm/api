import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import DocsSearch from '../DocsSearch';

const push = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push }),
}));

const INDEX = [
  {
    slug: 'getting-started/quickstart',
    title: 'Quickstart',
    section: 'getting-started',
    headings: ['Install the CLI', 'Convert a spec'],
    text: 'Install api with cargo install then run api convert on your OpenAPI spec.',
  },
  {
    slug: 'api-reference/formats',
    title: 'Output formats',
    section: 'api-reference',
    headings: ['Postman', 'Bruno'],
    text: 'Supported output formats for api collections.',
  },
];

describe('DocsSearch', () => {
  beforeEach(() => {
    push.mockClear();
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve({
          ok: true,
          json: () => Promise.resolve(INDEX),
        })
      )
    );
  });

  it('renders the search input', () => {
    render(<DocsSearch />);
    expect(
      screen.getByRole('searchbox', { name: 'Search docs' })
    ).toBeInTheDocument();
  });

  it('shows matching results after typing', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    expect(await screen.findByText('Quickstart')).toBeInTheDocument();
    expect(screen.getByText('Getting started')).toBeInTheDocument();
  });

  it('shows a no-results message for unmatched queries', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'zzzznope' } });
    await waitFor(() => {
      expect(screen.getByText(/No results/)).toBeInTheDocument();
    });
  });

  it('navigates on Enter when a result is active', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    await screen.findByText('Quickstart');
    fireEvent.keyDown(input, { key: 'ArrowDown' });
    fireEvent.keyDown(input, { key: 'Enter' });
    expect(push).toHaveBeenCalledWith('/docs/getting-started/quickstart/');
  });

  it('closes the panel on Escape', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    await screen.findByText('Quickstart');
    fireEvent.keyDown(input, { key: 'Escape' });
    expect(screen.queryByText('Quickstart')).not.toBeInTheDocument();
  });

  it('handles a failed index fetch gracefully', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.resolve({ ok: false, status: 500 }))
    );
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    await waitFor(() => {
      expect(screen.getByText(/No results/)).toBeInTheDocument();
    });
  });
});

describe('DocsSearch ranking and keyboard', () => {
  beforeEach(() => {
    push.mockClear();
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve({
          ok: true,
          json: () => Promise.resolve(INDEX),
        })
      )
    );
  });

  it('ranks heading matches and shows the heading', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'install' } });
    expect(await screen.findByText('Install the CLI')).toBeInTheDocument();
  });

  it('shows a body snippet for text matches', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'inventories' } });
    await waitFor(() => {
      expect(
        screen.getByText(/Supported output formats|inventories/i)
      ).toBeInTheDocument();
    });
  });

  it('does not navigate on Enter without an active result', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    await screen.findByText('Quickstart');
    fireEvent.keyDown(input, { key: 'Enter' });
    expect(push).not.toHaveBeenCalled();
  });

  it('moves active result up with ArrowUp', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'api' } });
    await screen.findAllByText(/api/i);
    fireEvent.keyDown(input, { key: 'ArrowDown' });
    fireEvent.keyDown(input, { key: 'ArrowDown' });
    fireEvent.keyDown(input, { key: 'ArrowUp' });
    fireEvent.keyDown(input, { key: 'Enter' });
    expect(push).toHaveBeenCalled();
  });

  it('closes the panel when a result link is clicked', async () => {
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    const link = await screen.findByRole('link', { name: /Quickstart/ });
    fireEvent.click(link);
    expect(
      screen.queryByRole('link', { name: /Quickstart/ })
    ).not.toBeInTheDocument();
  });
});

describe('DocsSearch edge cases', () => {
  beforeEach(() => {
    push.mockClear();
  });

  it('closes the panel on outside mousedown', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve({
          ok: true,
          json: () => Promise.resolve(INDEX),
        })
      )
    );
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'quickstart' } });
    await screen.findByText('Quickstart');
    fireEvent.mouseDown(document.body);
    expect(screen.queryByText('Quickstart')).not.toBeInTheDocument();
  });

  it('renders ellipsis snippets for long body matches and the overview entry', async () => {
    const longText = `intro ${'padding '.repeat(30)}needle term ${'tail '.repeat(30)} end`;
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve({
          ok: true,
          json: () =>
            Promise.resolve([
              {
                slug: '',
                title: 'Documentation',
                section: '',
                headings: [],
                text: 'api docs overview needle',
              },
              {
                slug: 'long',
                title: 'Long page',
                section: 'concepts',
                headings: [],
                text: longText,
              },
            ]),
        })
      )
    );
    render(<DocsSearch />);
    const input = screen.getByRole('searchbox', { name: 'Search docs' });
    fireEvent.focus(input);
    fireEvent.change(input, { target: { value: 'needle' } });
    await waitFor(() => {
      expect(screen.getAllByText(/needle/).length).toBeGreaterThan(0);
    });
    expect(document.body.textContent).toContain('…');
  });
});
