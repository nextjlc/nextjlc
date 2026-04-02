/* eslint.config.js */

import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

const tsFiles = ['**/*.{ts,tsx}']
const tsRules = tseslint.configs.recommended.reduce(
  (rules, config) => ({ ...rules, ...(config.rules ?? {}) }),
  {},
)
const tsPlugin = {
  meta: tseslint.plugin.meta,
  rules: tseslint.plugin.rules,
}
const reactHooksPlugin = {
  meta: reactHooks.meta,
  rules: reactHooks.rules,
}
const reactRefreshPlugin = {
  meta: reactRefresh.meta,
  rules: reactRefresh.rules,
}

export default defineConfig([
  globalIgnores(['dist']),
  {
    files: tsFiles,
    languageOptions: {
      ecmaVersion: 2020,
      parser: tseslint.parser,
      sourceType: 'module',
      globals: globals.browser,
    },
    plugins: {
      '@typescript-eslint': tsPlugin,
      'react-hooks': reactHooksPlugin,
      'react-refresh': reactRefreshPlugin,
    },
    rules: {
      ...js.configs.recommended.rules,
      ...tsRules,
      ...reactHooks.configs['recommended-latest'].rules,
      ...reactRefresh.configs.vite.rules,
    },
  },
])
