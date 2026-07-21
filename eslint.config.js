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
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },
  {
    // apps/landing is React + TanStack Start and carries its own flat config
    // (apps/landing/eslint.config.mjs); linting it here with the Vue rules
    // would only produce noise.
    ignores: [
      'apps/landing/',
      '**/src-tauri/',
      '**/dist/',
      '**/node_modules/',
      '**/.nuxt/',
      '**/.output/',
    ],
  },
  eslintConfigPrettier,
];
