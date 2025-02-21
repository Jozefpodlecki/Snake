/// <reference types="vite/types/importMeta.d.ts" />
/// <reference types="vitest" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tsconfigPaths from 'vite-tsconfig-paths'
import tailwindcss from '@tailwindcss/vite'
import mkcert from'vite-plugin-mkcert'

// https://vitejs.dev/config https://vitest.dev/config
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
    tsconfigPaths(),
    mkcert()
  ],
  test: {
    globals: true,
    environment: 'happy-dom',
    setupFiles: '.vitest/setup',
    include: ['**/test.{ts,tsx}']
  }
})