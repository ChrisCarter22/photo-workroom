import js from '@eslint/js';
import globals from 'globals';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import tseslint from 'typescript-eslint';
import { defineConfig } from 'eslint/config';

export default defineConfig([
  {
    ignores: ['**/dist/**', '**/target/**', '**/coverage/**', '**/node_modules/**'],
  },
  {
    files: ['apps/main-app/**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      ...tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2022,
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    files: [
      'apps/main-app/**/*.test.ts',
      'apps/main-app/**/*.test.tsx',
      'apps/main-app/**/*.integration.test.ts',
      'apps/main-app/**/*.integration.test.tsx',
      'apps/main-app/**/*.e2e.test.ts',
      'apps/main-app/**/*.e2e.test.tsx',
    ],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        beforeEach: 'readonly',
        describe: 'readonly',
        expect: 'readonly',
        it: 'readonly',
        vi: 'readonly',
      },
    },
  },
  {
    files: ['apps/main-app/vite.config.ts', 'apps/main-app/vitest*.config.ts'],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
]);
