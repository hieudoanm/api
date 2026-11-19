import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import SectionHead from '../SectionHead';

describe('SectionHead', () => {
  it('renders title and description', () => {
    render(<SectionHead title="Section" description="Details here." />);
    expect(screen.getByText('Section')).toBeInTheDocument();
    expect(screen.getByText('Details here.')).toBeInTheDocument();
  });
});
