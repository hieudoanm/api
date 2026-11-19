import type { Metadata } from 'next';
import type { FC } from 'react';
import ConverterApp from './converter';

export const metadata: Metadata = {
  title: 'Converter',
  description:
    'Convert OpenAPI 3.x JSON or YAML into Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya or curl collections entirely in your browser.',
  alternates: { canonical: '/app/' },
};

const AppPage: FC = () => <ConverterApp />;

export default AppPage;
