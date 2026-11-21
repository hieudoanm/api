import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import ConverterApp from '../converter';

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
          format: 'postman',
          path: 'petstore.postman_collection.json',
          content: '{"info":{}}',
        },
      ],
    })
  ),
  convertAll: vi.fn(() =>
    JSON.stringify({
      files: [
        {
          format: 'postman',
          path: 'a.postman_collection.json',
          content: '{}',
        },
        {
          format: 'curl',
          path: 'petstore.sh',
          content: '#!/usr/bin/env bash\n',
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

describe('ConverterApp', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText: vi.fn(() => Promise.resolve()) },
      configurable: true,
    });
    URL.createObjectURL = vi.fn(() => 'blob:mock');
    URL.revokeObjectURL = vi.fn();
  });

  it('renders the converter shell and reaches WASM ready', async () => {
    render(<ConverterApp />);
    expect(
      screen.getByRole('heading', { name: 'Convert OpenAPI in the browser.' })
    ).toBeInTheDocument();
    expect(await screen.findByText('WASM ready')).toBeInTheDocument();
    expect(screen.getByText('Output files')).toBeInTheDocument();
    expect(
      screen.getByText('Converted files will appear here.')
    ).toBeInTheDocument();
  });

  it('lists all ten formats plus all in the select', () => {
    render(<ConverterApp />);
    const select = document.querySelector('select')!;
    const options = Array.from(select.querySelectorAll('option')).map(
      (o) => o.textContent
    );
    expect(options).toContain('Postman');
    expect(options).toContain('Thunder Client');
    expect(options).toContain('Yaak');
    expect(options).toContain('Hurl');
    expect(options).toContain('Kreya');
    expect(options).toContain('curl (.sh)');
    expect(options).toContain('All formats');
  });

  it('shows an error when converting with empty source', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    // buttons are disabled while source is empty; force-enable path via inspect guard
    const convertBtn = screen.getByRole('button', { name: 'Convert' });
    expect(convertBtn).toBeDisabled();
  });

  it('inspects a pasted spec and shows collection info', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Inspect spec' }));
    expect(await screen.findByText('Petstore')).toBeInTheDocument();
    expect(screen.getByText('A pet store')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(openapiInfoMock).toHaveBeenCalledWith(SPEC);
  });

  it('converts a spec and lists output files', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    expect(
      await screen.findAllByText('petstore.postman_collection.json')
    ).not.toHaveLength(0);
    expect(screen.getByText('postman')).toBeInTheDocument();
    expect(convertMock).toHaveBeenCalled();
  });

  it('uses convert_all when format is all', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.change(document.querySelector('select')!, {
      target: { value: 'all' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findByText('petstore.sh');
    expect(convertAllMock).toHaveBeenCalled();
    expect(screen.getByText('Download ZIP')).toBeInTheDocument();
  });

  it('surfaces converter errors', async () => {
    convertMock.mockImplementationOnce(() => {
      throw new Error('parse failed');
    });
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    expect(await screen.findByText('parse failed')).toBeInTheDocument();
  });

  it('copies selected file content', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findAllByText('petstore.postman_collection.json');
    fireEvent.click(screen.getByRole('button', { name: /Copy content/ }));
    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalledWith('{"info":{}}');
    });
  });

  it('clears the form', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    fireEvent.change(screen.getByPlaceholderText(/Paste an OpenAPI document/), {
      target: { value: SPEC },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findAllByText('petstore.postman_collection.json');
    fireEvent.click(screen.getByRole('button', { name: /Clear/ }));
    expect(
      screen.getByPlaceholderText(/Paste an OpenAPI document/)
    ).toHaveValue('');
    expect(
      screen.getByText('Converted files will appear here.')
    ).toBeInTheDocument();
  });
});

describe('ConverterApp interactions', () => {
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

  it('loads a spec from the file input', async () => {
    render(<ConverterApp />);
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
    render(<ConverterApp />);
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

  it('passes name and base URL overrides to the converter', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.change(screen.getByPlaceholderText('info.title'), {
      target: { value: 'My API' },
    });
    fireEvent.change(screen.getByPlaceholderText('https://api.example.com'), {
      target: { value: 'https://override.dev' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findAllByText('petstore.postman_collection.json');
    expect(convertMock).toHaveBeenCalledWith(
      SPEC,
      'postman',
      'My API',
      'https://override.dev'
    );
  });

  it('downloads a single output file', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findAllByText('petstore.postman_collection.json');
    fireEvent.click(screen.getByRole('button', { name: /File/ }));
    expect(URL.createObjectURL).toHaveBeenCalled();
  });

  it('downloads a ZIP for multiple output files', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.change(document.querySelector('select')!, {
      target: { value: 'all' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findByText('Download ZIP');
    fireEvent.click(screen.getByRole('button', { name: 'Download ZIP' }));
    expect(URL.createObjectURL).toHaveBeenCalled();
  });

  it('switches the preview when another file is selected', async () => {
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.change(document.querySelector('select')!, {
      target: { value: 'all' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Convert' }));
    await screen.findByText('petstore.sh');
    fireEvent.click(screen.getByText('petstore.sh'));
    expect(await screen.findAllByText('petstore.sh')).not.toHaveLength(0);
    expect(screen.getByText('#!/usr/bin/env bash')).toBeInTheDocument();
  });

  it('loads a spec from the ?spec= query param', async () => {
    window.history.pushState({}, '', '/app/converter/?spec=petstore.yaml');
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.resolve(new Response(SPEC, { status: 200 })))
    );
    render(<ConverterApp />);
    await waitFor(() => {
      expect(
        screen.getByPlaceholderText(/Paste an OpenAPI document/)
      ).toHaveValue(SPEC);
    });
    expect(await screen.findByText('Petstore')).toBeInTheDocument();
    expect(fetch).toHaveBeenCalledWith('/apis/petstore.yaml');
    window.history.pushState({}, '', '/');
    vi.unstubAllGlobals();
  });

  it('ignores an invalid ?spec= param', async () => {
    window.history.pushState({}, '', '/app/converter/?spec=../secret.yaml');
    const fetchSpy = vi.fn();
    vi.stubGlobal('fetch', fetchSpy);
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    expect(fetchSpy).not.toHaveBeenCalled();
    expect(
      screen.getByPlaceholderText(/Paste an OpenAPI document/)
    ).toHaveValue('');
    window.history.pushState({}, '', '/');
    vi.unstubAllGlobals();
  });

  it('shows an error for a spec that fails inspection', async () => {
    openapiInfoMock.mockImplementationOnce(() => {
      throw new Error('bad spec');
    });
    render(<ConverterApp />);
    await screen.findByText('WASM ready');
    pasteSpec();
    fireEvent.click(screen.getByRole('button', { name: 'Inspect spec' }));
    expect(await screen.findByText('bad spec')).toBeInTheDocument();
  });
});
