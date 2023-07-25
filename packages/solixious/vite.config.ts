import path from 'path'
import { defineConfig } from 'vite'
import dts from 'vite-plugin-dts'
import inspect from 'vite-plugin-inspect'
import solidPlugin from 'vite-plugin-solid'

export default defineConfig(async (mode) => ({
  resolve: {
    alias: {
      '@bearbroidery/solixi': path.resolve(__dirname, './src'),
    },
  },
  build: {
    lib: {
      entry: './src/index.tsx',
      formats: ['es', 'cjs', 'umd'],
      fileName: 'index',
      name: 'solixi',
    },
    minify: false,
    rollupOptions: {
      external: ['solid-js', 'solid-js/web', 'solid-js/store', 'three', 'zustand', 'zustand/vanilla'],
    },
    polyfillDynamicImport: false,
  },
  plugins: [
    dts({
      insertTypesEntry: true,
    }),
    solidPlugin(),
    inspect(),
  ],
}))
