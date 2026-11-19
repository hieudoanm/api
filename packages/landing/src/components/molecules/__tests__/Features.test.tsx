import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import Features from '../Features';
import { LuTerminal } from 'react-icons/lu';

const features = [
  { title: 'Fast', body: 'Written in Rust.', icon: LuTerminal },
  { title: 'Local', body: 'No cloud.', icon: LuTerminal },
];

describe('Features', () => {
  it('renders the heading, description and feature cards', () => {
    render(
      <Features title="Why api" description="Because." features={features} />
    );
    expect(screen.getByText('Why api')).toBeInTheDocument();
    expect(screen.getByText('Because.')).toBeInTheDocument();
    expect(screen.getByText('Fast')).toBeInTheDocument();
    expect(screen.getByText('Written in Rust.')).toBeInTheDocument();
    expect(screen.getByText('Local')).toBeInTheDocument();
  });
});
