import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import libAssetsPlugin from '@laynezh/vite-plugin-lib-assets';

export default defineConfig(({ mode }) => {
    console.log(mode);
    const plugins = mode !== 'playground' ? [dts()] : [];
    return {
        build: {
            minify: false,
            lib: {
                formats: ['es'],
                entry: 'src/index.ts',
            },
            rollupOptions: {
                external: [
                    '@bearbroidery/bobbin-wasm-utils',
                    '@pixi/app',
                    '@pixi/assets',
                    '@pixi/core',
                    '@pixi/display',
                    '@pixi/mesh',
                    '@pixi/mesh-extras',
                    '@pixi/graphics',
                ],
            },
        },
        plugins,
    };
});
