import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import pluginVue from 'eslint-plugin-vue';
import eslintConfigPrettier from 'eslint-config-prettier';

export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },
  {
    files: ['apps/landing/**/*.vue', 'apps/landing/**/*.ts'],
    languageOptions: {
      globals: {
        useHead: 'readonly',
        useRoute: 'readonly',
        useRouter: 'readonly',
        useRuntimeConfig: 'readonly',
        useAsyncData: 'readonly',
        useFetch: 'readonly',
        ref: 'readonly',
        computed: 'readonly',
        reactive: 'readonly',
        onMounted: 'readonly',
        onUnmounted: 'readonly',
        watch: 'readonly',
        definePageMeta: 'readonly',
        navigateTo: 'readonly',
        useRelease: 'readonly',
        useI18n: 'readonly',
        useLocalePath: 'readonly',
        useSwitchLocalePath: 'readonly',
        useNuxtApp: 'readonly',
        useSeoMeta: 'readonly',
        useTrack: 'readonly',
      },
    },
  },
  {
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },
  {
    ignores: ['**/src-tauri/', '**/dist/', '**/node_modules/', '**/.nuxt/', '**/.output/'],
  },
  eslintConfigPrettier,
];
