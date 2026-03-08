import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

const tauriHost = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? '0.1.0'),
  },
  plugins: [react()],
  server: {
    host: tauriHost || false,
    port: 1420,
    strictPort: true,
  },
  preview: {
    host: tauriHost || false,
    port: 1420,
    strictPort: true,
  },
});
