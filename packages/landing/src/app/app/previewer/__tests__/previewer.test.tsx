import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import PreviewerApp from '../previewer';

const SPEC = JSON.stringify({
  openapi: '3.0.0',
  info: { title: 'Petstore', version: '1.0.0' },
  paths: {},
});

const wasmMocks = vi.hoisted(() => ({
  init: vi.fn(() => Promise.resolve()),
  openapiInfo: vi.fn(() =>
    JSON.stringify({
      name: 'Petstore',
      description: 'A pet store',
      version: '1.0.0',
      servers: ['https://api.example.com'],
      folders: [{ name: 'pet', requests: 2 }],
      requests: 2,
    })
  ),
  convert: vi.fn(() =>
    JSON.stringify({
      files: [
        {
          format: 'swagger-ui',
          path: 'petstore.swagger-ui.html',
          content: '<html><body>swagger</body></html>',
        },
      ],
    })
  ),
  convertAll: vi.fn(() =>
    JSON.stringify({
      files: [
        {
          format: 'swagger-ui',
          path: 'petstore.swagger-ui.html',
          content: '<html>ui</html>',
        },
        {
          format: 'redoc',
          path: 'petstore.redoc.html',
          content: '<html>redoc</html>',
        },
        {
          format: 'postman',
          path: 'petstore.postman_collection.json',
          content: '{}',
        },
      ],
    })
  ),
}));

const openapiInfoMock = wasmMocks.openapiInfo;
const convertMock = wasmMocks.convert;
const convertAllMock = wasmMocks.convertAll;

vi.mock('@api/wasm', () => ({
  default: wasmMocks.init,
  init: wasmMocks.init,
  convert: wasmMocks.convert,
  convert_all: wasmMocks.convertAll,
  openapi_info: wasmMocks.openapiInfo,
}));

describe('PreviewerApp', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText: vi.fn(() => Promise.resolve()) },
      configurable: true,
    });
    URL.createObjectURL = vi.fn(() => 'blob:mock');
    URL.revokeObjectURL = vi.fn();
  });

  const pasteSpec = () => {
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
  };

  it('renders the previewer shell and reaches WASM ready', async () => {
    render(<PreviewerApp />);
    expect(
      screen.getByRole('heading', {
        name: 'Preview API docs in the browser.',
      })
    ).toBeInTheDocument();
    expect(await screen.findByText('WASM ready')).toBeInTheDocument();
    expect(screen.getByText('Preview files')).toBeInTheDocument();
    expect(
      screen.getByText('Generated HTML previews will appear here.')
    ).toBeInTheDocument();
  });

  it('lists the five preview formats plus all', () => {
    render(<PreviewerApp />);
    const select = document.querySelector('select')!;
    const options = Array.from(select.querySelectorAll('option')).map(
      (o) => o.textContent
    );
    expect(options).toEqual([
      'Swagger UI',
      'ReDoc',
      'Scalar',
      'Stoplight Elements',
      'RapiDoc',
      'All previews',
    ]);
  });

  it('inspects a pasted spec and shows collection info', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Inspect spec' }));
    expect(await screen.findByText('Petstore')).toBeInTheDocument();
    expect(screen.getByText('A pet store')).toBeInTheDocument();
    expect(openapiInfoMock).toHaveBeenCalledWith(SPEC);
  });

  it('converts a spec and shows the HTML preview', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    expect(
      await screen.findAllByText('petstore.swagger-ui.html')
    ).not.toHaveLength(0);
    expect(convertMock).toHaveBeenCalledWith(SPEC, 'swagger-ui', '', '');
    expect(document.querySelector('iframe')).toBeInTheDocument();
  });

  it('filters non-preview files when format is all', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.change(document.querySelector('select')!, {
      target: { value: 'all' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    await screen.findByText('petstore.redoc.html');
    expect(convertAllMock).toHaveBeenCalled();
    expect(
      screen.queryByText('petstore.postman_collection.json')
    ).not.toBeInTheDocument();
  });

  it('passes the collection name override', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.change(screen.getByPlaceholderText('info.title'), {
      target: { value: 'My Docs' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    await screen.findAllByText('petstore.swagger-ui.html');
    expect(convertMock).toHaveBeenCalledWith(SPEC, 'swagger-ui', 'My Docs', '');
  });

  it('surfaces converter errors', async () => {
    convertMock.mockImplementationOnce(() => {
      throw new Error('parse failed');
    });
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    expect(await screen.findByText('parse failed')).toBeInTheDocument();
  });

  it('shows an error when the converter produces no preview files', async () => {
    convertMock.mockImplementationOnce(() =>
      JSON.stringify({
        files: [{ format: 'postman', path: 'x.json', content: '{}' }],
      })
    );
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    expect(
      await screen.findByText('converter produced no files')
    ).toBeInTheDocument();
  });

  it('shows an error when inspection fails', async () => {
    openapiInfoMock.mockImplementationOnce(() => {
      throw new Error('bad spec');
    });
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Inspect spec' }));
    expect(await screen.findByText('bad spec')).toBeInTheDocument();
  });

  it('copies selected file content', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    await screen.findAllByText('petstore.swagger-ui.html');
    fireEvent.click(screen.getByRole('button', { name: /Copy/ }));
    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
        '<html><body>swagger</body></html>'
      );
    });
  });

  it('clears the form', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    await screen.findAllByText('petstore.swagger-ui.html');
    fireEvent.click(screen.getByRole('button', { name: /Clear/ }));
    expect(
      screen.getByPlaceholderText(/Paste an OpenAPI document/)
    ).toHaveValue('');
    expect(
      screen.getByText('Generated HTML previews will appear here.')
    ).toBeInTheDocument();
  });

  it('loads a spec from the file input', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    const input = document.querySelector('input[type="file"]')!;
    const file = new File([SPEC], 'petstore.json', {
      type: 'application/json',
    });
    fireEvent.change(input, { target: { files: [file] } });
    await waitFor(() => {
      expect(
        screen.getByPlaceholderText(/Paste an OpenAPI document/)
      ).toHaveValue(SPEC);
    });
  });

  it('loads a spec from a drop event', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    const file = new File([SPEC], 'petstore.yaml', { type: 'text/yaml' });
    const zone = document.querySelector('.border-dashed')!;
    fireEvent.dragOver(zone);
    fireEvent.drop(zone, { dataTransfer: { files: [file] } });
    await waitFor(() => {
      expect(
        screen.getByPlaceholderText(/Paste an OpenAPI document/)
      ).toHaveValue(SPEC);
    });
  });

  it('loads a spec from the ?spec= query param', async () => {
    window.history.pushState({}, '', '/app/previewer/?spec=petstore.yaml');
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.resolve(new Response(SPEC, { status: 200 })))
    );
    render(<PreviewerApp />);
    await waitFor(() => {
      expect(
        screen.getByPlaceholderText(/Paste an OpenAPI document/)
      ).toHaveValue(SPEC);
    });
    expect(await screen.findByText('Petstore')).toBeInTheDocument();
    expect(fetch).toHaveBeenCalledWith('/apis/yaml/petstore.yaml');
    window.history.pushState({}, '', '/');
    vi.unstubAllGlobals();
  });

  it('downloads a generated file', async () => {
    render(<PreviewerApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Generate preview' }));
    await screen.findAllByText('petstore.swagger-ui.html');
    fireEvent.click(screen.getByRole('button', { name: /File/ }));
    expect(URL.createObjectURL).toHaveBeenCalled();
  });
});
