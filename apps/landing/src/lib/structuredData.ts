import type { Locale } from '~/i18n/locale'
import { REPO, faqItems } from '~/content/landing'
import { dictionaries } from '~/i18n'

export const SITE_URL = 'https://craite.clhub.fr'

// SoftwareApplication plus FAQPage, generated from the content modules so the
// structured data can never contradict what the page says. The FAQ block is
// what earns the expandable answers in search results.
export function landingJsonLd(locale: Locale) {
  const m = dictionaries[locale]

  return {
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@type': 'SoftwareApplication',
        name: 'craite',
        url: SITE_URL,
        applicationCategory: 'MultimediaApplication',
        operatingSystem: 'Windows, macOS, Linux',
        description: m.hero.intro,
        codeRepository: REPO,
        isAccessibleForFree: true,
        offers: {
          '@type': 'Offer',
          price: '0',
          priceCurrency: 'EUR',
        },
        author: {
          '@type': 'Person',
          name: 'Cédric Lucchese',
          url: 'https://clhub.fr',
        },
      },
      {
        '@type': 'FAQPage',
        mainEntity: faqItems.map((id) => ({
          '@type': 'Question',
          name: m.faqItems[id].question,
          acceptedAnswer: {
            '@type': 'Answer',
            text: m.faqItems[id].answer,
          },
        })),
      },
    ],
  }
}
