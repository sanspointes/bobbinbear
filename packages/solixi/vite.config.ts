import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import inspect from 'vite-plugin-inspect'

export default defineConfig({
  plugins: [solid(), inspect()],
})
