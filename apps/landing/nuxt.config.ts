export default defineNuxtConfig({
  modules: ['@nuxt/ui'],

  runtimeConfig: {
    githubToken: process.env.GITHUB_TOKEN,
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
