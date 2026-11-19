import type { Metadata } from 'next';
import type { FC } from 'react';
import PreviewerApp from './previewer';

export const metadata: Metadata = {
  title: 'Previewer',
  description:
    'Render OpenAPI 3.x specs with Swagger UI, ReDoc, Scalar, Stoplight Elements or RapiDoc entirely in your browser.',
  alternates: { canonical: '/app/previewer/' },
};

const PreviewerPage: FC = () => <PreviewerApp />;

export default PreviewerPage;
