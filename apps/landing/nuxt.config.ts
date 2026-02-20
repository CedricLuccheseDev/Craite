export default defineNuxtConfig({
  modules: ['@nuxt/ui'],

  ui: {
    colors: {
      primary: 'orange',
      neutral: 'zinc',
    },
  },

  colorMode: {
    preference: 'dark',
    fallback: 'dark',
  },

  compatibilityDate: '2025-01-01',
});
