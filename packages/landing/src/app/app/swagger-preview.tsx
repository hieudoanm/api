'use client';

import { useEffect, useMemo, useState } from 'react';
import type { ComponentType, FC } from 'react';
import { parse } from 'yaml';
import 'swagger-ui-react/swagger-ui.css';

interface SwaggerUIProps {
  spec: Record<string, unknown>;
  docExpansion?: 'none' | 'list' | 'full';
  deepLinking?: boolean;
  displayRequestDuration?: boolean;
}

type SwaggerUIComponent = ComponentType<SwaggerUIProps>;

let swaggerUI: SwaggerUIComponent | null = null;
let swaggerUIPromise: Promise<SwaggerUIComponent> | null = null;

const loadSwaggerUI = (): Promise<SwaggerUIComponent> => {
  if (!swaggerUIPromise) {
    swaggerUIPromise = import('swagger-ui-react')
      .then((mod) => {
        swaggerUI = mod.default as unknown as SwaggerUIComponent;
        return swaggerUI;
      })
      .catch((error: unknown) => {
        swaggerUIPromise = null;
        throw error;
      });
  }
  return swaggerUIPromise;
};

const errorText = (error: unknown): string =>
  error instanceof Error ? error.message : String(error);

const parseSpec = (
  source: string
): { spec?: Record<string, unknown>; error?: string } => {
  try {
    const value = parse(source) as unknown;
    if (!value || typeof value !== 'object' || Array.isArray(value)) {
      return { error: 'Swagger UI needs an OpenAPI object.' };
    }
    return { spec: value as Record<string, unknown> };
  } catch (error) {
    return { error: errorText(error) };
  }
};

const SwaggerPreview: FC<{ source: string }> = ({ source }) => {
  const [Component, setComponent] = useState<SwaggerUIComponent | null>(
    swaggerUI
  );
  const [loadError, setLoadError] = useState('');
  const parsed = useMemo(() => parseSpec(source), [source]);
  const spec = parsed.spec;

  useEffect(() => {
    if (!spec) return;
    let cancelled = false;
    loadSwaggerUI()
      .then((loaded) => {
        if (!cancelled) setComponent(() => loaded);
      })
      .catch((error: unknown) => {
        if (!cancelled) setLoadError(errorText(error));
      });
    return () => {
      cancelled = true;
    };
  }, [spec]);

  if (!source.trim()) {
    return (
      <div className="border-base-content/10 text-base-content/50 rounded-2xl border border-dashed p-8 text-center text-sm">
        Paste or drop an OpenAPI spec to preview it in Swagger UI.
      </div>
    );
  }

  if (parsed.error || !spec) {
    return (
      <div className="alert alert-error text-sm">
        Invalid OpenAPI document: {parsed.error ?? 'unable to parse spec'}
      </div>
    );
  }

  if (loadError) {
    return (
      <div className="alert alert-error text-sm">
        Failed to load Swagger UI: {loadError}
      </div>
    );
  }

  if (!Component) {
    return (
      <div className="border-base-content/10 bg-base-100 text-base-content/60 rounded-2xl border p-8 text-center text-sm">
        Loading Swagger UI…
      </div>
    );
  }

  return (
    <div className="border-base-content/10 overflow-hidden rounded-2xl border bg-white">
      <Component
        spec={spec}
        docExpansion="list"
        deepLinking
        displayRequestDuration
      />
    </div>
  );
};

export default SwaggerPreview;
