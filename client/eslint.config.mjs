import eslintParserTs from '@typescript-eslint/parser';
import eslintPluginTs from '@typescript-eslint/eslint-plugin';
import eslintPluginVue from 'eslint-plugin-vue';
import prettierConfig from 'eslint-config-prettier';
import vueTsEslintConfig from '@vue/eslint-config-typescript';

export default [
  {
    files: ['**/*.{ts,tsx,vue,js,jsx}'],
    languageOptions: {
      parser: eslintParserTs,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    plugins: {
      vue: eslintPluginVue,
      '@typescript-eslint': eslintPluginTs,
    },
    rules: {
      ...prettierConfig.rules,
    },
  },
  ...vueTsEslintConfig()
];
