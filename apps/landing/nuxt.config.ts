export default defineNuxtConfig({
  modules: ['@nuxt/ui'],

  runtimeConfig: {
    releaseUrl: process.env.RELEASE_URL || 'https://dl.craite.clhub.fr',
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
