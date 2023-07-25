import type { StorybookConfig } from "storybook-solidjs-vite";
import { mergeConfig } from 'vite';
import solidPlugin from "vite-plugin-solid";
import { DOMElements, SVGElements } from "solid-js/web/dist/dev.cjs";

import { join, dirname } from "path";

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 */
function getAbsolutePath(value: string): any {
  return dirname(require.resolve(join(value, "package.json")));
}
const config: StorybookConfig = {
  stories: ["../src/**/*.mdx", "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
  addons: [
    getAbsolutePath("@storybook/addon-links"),
    getAbsolutePath("@storybook/addon-essentials"),
    getAbsolutePath("@storybook/addon-interactions"),
  ],
  framework: {
    name: getAbsolutePath("storybook-solidjs-vite"),
    options: {},
  },
  docs: {
    autodocs: "tag",
  },
  async viteFinal(config) {
    // Merge custom configuration into the default config
    return mergeConfig(config, {
      plugins: [
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
      ]
    });
  },
};
export default config;
