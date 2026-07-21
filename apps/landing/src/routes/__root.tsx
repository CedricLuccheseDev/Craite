import {
  HeadContent,
  Outlet,
  Scripts,
  createRootRoute,
} from '@tanstack/react-router'
import type { Locale } from '~/i18n/locale'
import type { ReactNode } from 'react'
import appCss from '~/styles/app.css?url'
import { NotFoundScreen } from '~/components/ErrorScreens'
import { LocaleProvider, dictionaries } from '~/i18n'
import { getLocale } from '~/i18n/locale'
import { SITE_URL, landingJsonLd } from '~/lib/structuredData'

export const Route = createRootRoute({
  // The locale never changes within a session — don't refetch on navigation.
  loader: () => getLocale(),
  staleTime: Infinity,
  head: ({ loaderData }) => {
    const locale: Locale = loaderData ?? 'fr'
    const m = dictionaries[locale]
    const title = `craite, ${m.hero.title}`
    const description = m.hero.intro

    return {
      meta: [
        { charSet: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { title },
        { name: 'description', content: description },
        {
          name: 'robots',
          content: 'index, follow, max-image-preview:large, max-snippet:-1',
        },
        { name: 'theme-color', content: '#f4f4f1' },
        { property: 'og:site_name', content: 'craite' },
        { property: 'og:title', content: title },
        { property: 'og:description', content: description },
        { property: 'og:type', content: 'website' },
        { property: 'og:url', content: SITE_URL },
        { property: 'og:locale', content: locale === 'fr' ? 'fr_FR' : 'en_US' },
        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:title', content: title },
        { name: 'twitter:description', content: description },
      ],
      links: [
        { rel: 'stylesheet', href: appCss },
        { rel: 'canonical', href: `${SITE_URL}/` },
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
        // Fonts are self-hosted; preload only the faces the first screen paints.
        {
          rel: 'preload',
          as: 'font',
          type: 'font/woff2',
          href: '/fonts/space-grotesk-700.woff2',
          crossOrigin: 'anonymous',
        },
        {
          rel: 'preload',
          as: 'font',
          type: 'font/woff2',
          href: '/fonts/inter-400.woff2',
          crossOrigin: 'anonymous',
        },
      ],
    }
  },
  notFoundComponent: NotFoundScreen,
  component: RootComponent,
})

function RootComponent() {
  const locale = Route.useLoaderData()

  return (
    <LocaleProvider locale={locale}>
      <RootDocument locale={locale}>
        <Outlet />
      </RootDocument>
    </LocaleProvider>
  )
}

function RootDocument({
  locale,
  children,
}: {
  locale: Locale
  children: ReactNode
}) {
  return (
    <html lang={locale}>
      <head>
        <HeadContent />
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify(landingJsonLd(locale)),
          }}
        />
      </head>
      <body>
        {children}
        <Scripts />
      </body>
    </html>
  )
}
