import prettierConfig from '@vue/eslint-config-prettier';
import typescriptEslint from '@typescript-eslint/eslint-plugin';
import typescriptParser from '@typescript-eslint/parser';
import { configs as vueConfigs } from 'eslint-plugin-vue';

export default [
  {
    files: ['**/*.js', '**/*.ts', '**/*.vue'],
    languageOptions: {
      parser: typescriptParser,
      ecmaVersion: 'latest',
      sourceType: 'module',
      parserOptions: {
        ecmaFeatures: { jsx: true },
      },
    },
    plugins: {
      vue: vueConfigs,
      '@typescript-eslint': typescriptEslint,
    },
    rules: {
      ...vueConfigs['vue3-essential'].rules,
      ...typescriptEslint.configs.recommended.rules,
      ...typescriptEslint.configs['recommended-requiring-type-checking'].rules,
      ...prettierConfig.rules,
    },
  },
];
