import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import ErrorPage from '../error';
import GlobalError from '../global-error';
import NotFound from '../not-found';
import Loading from '../loading';

describe('Error', () => {
  it('renders the message and calls reset', () => {
    const reset = vi.fn();
    render(<ErrorPage error={new Error('boom')} reset={reset} />);
    expect(screen.getByText('boom')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Try again' }));
    expect(reset).toHaveBeenCalled();
  });
});

describe('GlobalError', () => {
  it('renders the message and calls reset', () => {
    const reset = vi.fn();
    render(<GlobalError error={new Error('fatal')} reset={reset} />);
    expect(screen.getByText('fatal')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Try again' }));
    expect(reset).toHaveBeenCalled();
  });
});

describe('NotFound', () => {
  it('renders the 404 message and home link', () => {
    render(<NotFound />);
    expect(screen.getByText('404')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Back to api' })).toHaveAttribute(
      'href',
      '/'
    );
  });
});

describe('Loading', () => {
  it('renders a spinner', () => {
    const { container } = render(<Loading />);
    expect(container.querySelector('.loading')).toBeInTheDocument();
  });
});
