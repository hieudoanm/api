import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import type { FC, ReactNode } from 'react';
import Footer from '@/components/molecules/Footer';
import Navbar from '@/components/molecules/Navbar';
import './globals.css';

const THEME_INIT_SCRIPT = `(function(){try{var t=localStorage.getItem('api-theme');if(!t){t=window.matchMedia('(prefers-color-scheme: dark)').matches?'api-dark':'api';}document.documentElement.setAttribute('data-theme',t);}catch(e){}})();`;

const inter = Inter({ subsets: ['latin'] });

const GITHUB = 'https://github.com/hieudoanm/api';
const LOGO =
  process.env.NODE_ENV === 'production' ? '/api/logo.svg' : '/logo.svg';
const NAV_LINKS = [
  { label: 'Features', href: '/#features' },
  { label: 'Formats', href: '/#formats' },
  { label: 'Quickstart', href: '/#quickstart' },
  { label: 'Apps', href: '/app/' },
  { label: 'Docs', href: '/docs/' },
  { label: 'Downloads', href: '/downloads/' },
];

export const metadata: Metadata = {
  metadataBase: new URL('https://hieudoanm.github.io/api/'),
  title: {
    default: 'API — OpenAPI to API client collections',
    template: '%s — api',
  },
  description:
    'Convert OpenAPI 3.x specs into ready-to-use collections for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — one Rust CLI, no cloud, no accounts.',
  keywords: [
    'OpenAPI',
    'Postman',
    'Bruno',
    'Insomnia',
    'Hoppscotch',
    'REST Client',
    'API client',
    'collection converter',
    'Rust',
    'CLI',
  ],
  authors: [{ name: 'hieudoan', url: 'https://github.com/hieudoanm/api' }],
  category: 'software',
  openGraph: {
    siteName: 'API',
    type: 'website',
    title: 'API — OpenAPI to API client collections',
    description:
      'Turn any OpenAPI 3.x spec into Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya or curl collections — one Rust CLI.',
    url: 'https://hieudoanm.github.io/api/',
    images: ['/og.png'],
  },
  twitter: { card: 'summary_large_image', images: ['/og.png'] },
};

const JSON_LD = {
  '@context': 'https://schema.org',
  '@graph': [
    {
      '@type': 'WebSite',
      name: 'API',
      url: 'https://hieudoanm.github.io/api/',
      description:
        'Convert OpenAPI 3.x specs into ready-to-use collections for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — one Rust CLI, no cloud, no accounts.',
    },
    {
      '@type': 'SoftwareApplication',
      name: 'API',
      applicationCategory: 'DeveloperApplication',
      description:
        'Convert OpenAPI 3.x specs into ready-to-use collections for Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client, Yaak, Hurl, Kreya and curl — one Rust CLI, no cloud, no accounts.',
      url: 'https://hieudoanm.github.io/api/',
      codeRepository: 'https://github.com/hieudoanm/api',
      offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
    },
  ],
};

interface RootLayoutProps {
  children: ReactNode;
}

const RootLayout: FC<RootLayoutProps> = ({ children }) => (
  <html
    lang="en"
    data-theme="api"
    suppressHydrationWarning
    className={`scroll-smooth antialiased motion-reduce:scroll-auto ${inter.className}`}>
    <body className="bg-base-100 text-base-content m-0 leading-relaxed">
      <script dangerouslySetInnerHTML={{ __html: THEME_INIT_SCRIPT }} />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(JSON_LD) }}
      />
      <Navbar name="API" logoSrc={LOGO} links={NAV_LINKS} githubUrl={GITHUB} />
      {children}
      <Footer
        name="API"
        tagline="OpenAPI to API client collections · Rust · MIT"
        githubUrl={GITHUB}
      />
    </body>
  </html>
);

export default RootLayout;
