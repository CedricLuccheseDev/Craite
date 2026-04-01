import { resolve } from 'path';
import { config } from 'dotenv';

config({ path: resolve(__dirname, '../../.env') });

export default defineNuxtConfig({
  modules: ['@nuxt/ui', '@nuxtjs/i18n'],

  i18n: {
    locales: [
      { code: 'en', language: 'en', file: 'en.json', name: 'English' },
      { code: 'fr', language: 'fr', file: 'fr.json', name: 'Français' },
    ],
    defaultLocale: 'en',
    langDir: 'locales',
    strategy: 'prefix_except_default',
  },

  runtimeConfig: {
    releaseUrl: process.env.RELEASE_URL || 'https://dl.craite.clhub.fr',
    public: {
      posthogKey: process.env.POSTHOG_KEY || '',
      posthogHost: process.env.POSTHOG_HOST || 'https://us.i.posthog.com',
    },
  },

  css: ['~/assets/css/main.css'],

  fonts: {
    families: [
      { name: 'Inter', provider: 'google' },
      { name: 'Outfit', provider: 'google' },
    ],
    // Disable unused providers to avoid initialization errors
    providers: {
      fontshare: false,
    },
  },

  colorMode: {
    preference: 'dark',
    fallback: 'dark',
  },

  compatibilityDate: '2025-01-01',
});
