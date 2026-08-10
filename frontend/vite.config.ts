import { defineConfig } from 'vite';

/**
 * Конфигурация Vite для фронтенда визуализатора Event Loop.
 * Настроена поддержка WASM-модулей (для интерпретатора на Rust).
 */
export default defineConfig({
  // Относительный base — позволяет разворачивать сборку в подпапке
  // (например, на GitHub Pages по адресу /<repo-name>/), а также в корне домена.
  base: './',
  server: {
    port: 5173,
    open: true,
  },
  build: {
    target: 'es2022',
    outDir: 'dist',
  },
  optimizeDeps: {
    exclude: ['@webcomponents/webcomponentsjs'],
  },
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['src/**/*.test.ts'],
  },
});