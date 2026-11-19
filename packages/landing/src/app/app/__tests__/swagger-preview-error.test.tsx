import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import SwaggerPreview from '../swagger-preview';

vi.mock('swagger-ui-react', () => {
  throw new Error('chunk load failed');
});

const VALID =
  'openapi: 3.0.0\ninfo:\n  title: Petstore\n  version: "1"\npaths: {}\n';

describe('SwaggerPreview load failure', () => {
  it('shows an error when Swagger UI cannot be loaded', async () => {
    render(<SwaggerPreview source={VALID} />);
    expect(
      await screen.findByText(/Failed to load Swagger UI/)
    ).toBeInTheDocument();
    expect(screen.getByText(/mocking a module/)).toBeInTheDocument();
  });
});
