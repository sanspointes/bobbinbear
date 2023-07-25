import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import inspect from "vite-plugin-inspect";
import dts from "vite-plugin-dts";
import { DOMElements, SVGElements } from "solid-js/web/dist/dev.cjs";

export default defineConfig({
  build: {
    lib: {
      entry: "./src/index.tsx",
      formats: ["es", "cjs", "umd"],
      fileName: "index",
      name: "SolidThree",
    },
    minify: false,
    rollupOptions: {
      external: [
        "solid-js",
        "solid-js/web",
        "solid-js/store",
        "three",
        "zustand",
        "zustand/vanilla",
      ],
    },
  },
  plugins: [
    dts({
      entryRoot: 'src',
      tsconfigPath: './tsconfig.json',
    }),
    // mdx({
    //   transformMDX: (code) => {
    //     return code.replace(/<\!--[a-zA-Z\.\s]+-->/g, ` `);
    //   },
    //   xdm: {
    //     remarkPlugins: [(await import("remark-gfm")).default],
    //   },
    // }),
    // for the playground, we need to be able to use the renderer from the src itself
    solidPlugin({
      solid: {
        moduleName: "solid-js/web",
        // @ts-ignore
        generate: "dynamic",
        renderers: [
          {
            name: "dom",
            moduleName: "solid-js/web",
            elements: [...DOMElements.values(), ...SVGElements.values()],
          },
          {
            name: "universal",
            moduleName: "/src/renderer.tsx",
            elements: [],
          },
        ],
      },
    }),
    inspect(),
  ],
});
