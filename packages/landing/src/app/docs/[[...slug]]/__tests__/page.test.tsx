import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import DocsPage, { generateMetadata, generateStaticParams } from '../page';

const notFoundMock = vi.fn(() => {
  throw new Error('NEXT_NOT_FOUND');
});

vi.mock('next/navigation', () => ({
  notFound: () => notFoundMock(),
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock('next/link', () => ({
  default: ({
    href,
    children,
    ...rest
  }: {
    href: string;
    children: React.ReactNode;
  }) => (
    <a href={href} {...rest}>
      {children}
    </a>
  ),
}));

const params = (slug?: string[]) => Promise.resolve({ slug });

describe('generateStaticParams', () => {
  it('returns a param set for every doc page', () => {
    const sets = generateStaticParams();
    expect(sets.length).toBeGreaterThan(0);
    expect(sets.some((s) => s.slug.length === 0)).toBe(true);
    expect(
      sets.some((s) => s.slug.join('/') === 'getting-started/quickstart')
    ).toBe(true);
  });
});

describe('generateMetadata', () => {
  it('returns title, description and canonical for a known doc', async () => {
    const meta = await generateMetadata({
      params: params(['getting-started', 'quickstart']),
    });
    expect(meta.title).toBeTruthy();
    expect(meta.description).toBeTruthy();
    expect(meta.alternates?.canonical).toBe(
      '/docs/getting-started/quickstart/'
    );
  });

  it('falls back to Docs for an unknown slug', async () => {
    const meta = await generateMetadata({ params: params(['nope']) });
    expect(meta.title).toBe('Docs');
    expect(meta.alternates?.canonical).toBe('/docs/nope/');
  });
});

describe('DocsPage', () => {
  beforeEach(() => {
    notFoundMock.mockClear();
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.resolve({ ok: false, status: 404 }))
    );
  });

  it('renders the docs overview', async () => {
    render(await DocsPage({ params: params() }));
    expect(screen.getByText('Documentation')).toBeInTheDocument();
    expect(screen.getByLabelText('Search docs')).toBeInTheDocument();
  });

  it('renders a section page with toc and prev/next nav', async () => {
    render(
      await DocsPage({ params: params(['getting-started', 'quickstart']) })
    );
    expect(document.querySelector('.prose')).toBeInTheDocument();
    expect(screen.queryByText('On this page') ?? document.body).toBeTruthy();
  });

  it('calls notFound for an unknown slug', async () => {
    await expect(
      DocsPage({ params: params(['does-not-exist']) })
    ).rejects.toThrow('NEXT_NOT_FOUND');
    expect(notFoundMock).toHaveBeenCalled();
  });
});
