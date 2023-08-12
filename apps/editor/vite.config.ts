import { defineConfig, type Plugin } from "vite";
import solid from 'vite-plugin-solid'
import inspect from "vite-plugin-inspect";
import solidDevtoolsPlugin from "solid-devtools/vite";

export default defineConfig(({mode})  => {
  const plugins = [
    inspect(),
    solid(),
  ];
  if (mode === "development") {
    plugins.push(solidDevtoolsPlugin({ autoname: true }) as Plugin);
  }

  return {
    plugins,
    build: {
      emptyOutDir: true,
      minify: false,
      sourcemap: true,
    },
  }
})
