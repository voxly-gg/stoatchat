import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import type { ScalarOptions } from '@scalar/docusaurus';

const config: Config = {
  title: 'Voxly Developers',
  tagline: 'Developer documentation for Voxly',
  favicon: 'https://voxly.gg/favicon.svg',

  future: {
    v4: true,
  },

  url: 'https://developers.voxly.gg',
  baseUrl: '/',

  organizationName: 'voxly-gg',
  projectName: 'voxly-backend',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: './sidebars.ts',
          editUrl:
            'https://github.com/voxly-gg/stoatchat/tree/main/docs/',
        },
      } satisfies Preset.Options,
    ],
  ],

  plugins: [
    [
      '@scalar/docusaurus',
      {
        label: 'API Reference',
        route: '/api-reference',
        showNavLink: true,
        configuration: {
          url: 'https://voxly.gg/api/openapi.json',
        },
      } as ScalarOptions,
    ],
    [
      '@docusaurus/plugin-client-redirects',
      {
        fromExtensions: ['html', 'htm'],
        redirects: [
          // legacy docs website (stoatchat/developer-wiki)
          {
            from: '/developers/api/reference.html',
            to: '/api-reference',
          },
          {
            from: '/contrib.html',
            to: '/developing/contrib',
          },
          {
            from: '/contrib',
            to: '/developing/contrib',
          },
        ],
      }
    ],
  ],

  themeConfig: {
    // image: 'img/docusaurus-social-card.jpg',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Voxly Developers',
      logo: {
        alt: 'Voxly',
        src: 'https://voxly.gg/favicon.svg',
      },
      items: [
        {
          type: 'doc',
          docId: 'index',
          label: 'Docs'
        },
        {
          href: 'https://github.com/voxly-gg',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Developers',
          items: [
            {
              label: 'Source Code',
              href: 'https://github.com/voxly-gg'
            },
            {
              label: 'Help Translate',
              href: 'https://translate.voxly.gg'
            },
          ],
        },
        {
          title: 'Team',
          items: [
            {
              label: 'About',
              href: 'https://voxly.gg/about'
            },
            {
              label: 'Blog and Changelogs',
              href: 'https://voxly.gg/updates'
            },
            {
              label: 'Contact',
              href: 'https://support.voxly.gg'
            },
          ],
        },
        {
          title: 'Voxly on Socials',
          items: [
            {
              label: 'Bluesky',
              href: 'https://bsky.app/profile/voxly.gg'
            },
            {
              label: 'Reddit',
              href: 'https://reddit.com/r/voxly'
            },
            {
              label: 'Voxly Server',
              href: 'https://voxly.gg/invite'
            },
          ],
        },
        {
          title: 'Legal',
          items: [
            {
              label: 'Community Guidelines',
              href: 'https://voxly.gg/legal/community-guidelines'
            },
            {
              label: 'Terms of Service',
              href: 'https://voxly.gg/legal/terms'
            },
            {
              label: 'Privacy Policy',
              href: 'https://voxly.gg/legal/privacy'
            },
            {
              label: 'Imprint',
              href: 'https://voxly.gg/legal/imprint'
            },
          ],
        },
      ],
      copyright: `© Voxly, ${new Date().getFullYear()}`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
