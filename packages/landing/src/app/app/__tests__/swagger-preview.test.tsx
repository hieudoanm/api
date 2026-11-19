import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import SwaggerPreview from '../swagger-preview';

vi.mock('swagger-ui-react', () => ({
  default: ({ spec }: { spec: Record<string, unknown> }) => (
    <div data-testid="swagger-ui">{spec.info ? 'spec loaded' : 'empty'}</div>
  ),
}));

const VALID =
  'openapi: 3.0.0\ninfo:\n  title: Petstore\n  version: "1"\npaths: {}\n';

describe('SwaggerPreview', () => {
  it('shows a placeholder for empty source', () => {
    render(<SwaggerPreview source="   " />);
    expect(
      screen.getByText(/Paste or drop an OpenAPI spec to preview/)
    ).toBeInTheDocument();
  });

  it('shows an error for invalid YAML', () => {
    render(<SwaggerPreview source=":::not yaml [" />);
    expect(screen.getByText(/Invalid OpenAPI document/)).toBeInTheDocument();
  });

  it('shows an error for non-object YAML', () => {
    render(<SwaggerPreview source="- just\n- a\n- list\n" />);
    expect(screen.getByText(/Invalid OpenAPI document/)).toBeInTheDocument();
  });

  it('loads Swagger UI for a valid spec', async () => {
    render(<SwaggerPreview source={VALID} />);
    expect(
      await screen.findByTestId('swagger-ui', {}, { timeout: 5000 })
    ).toBeInTheDocument();
    expect(screen.getByText('spec loaded')).toBeInTheDocument();
  });
});
