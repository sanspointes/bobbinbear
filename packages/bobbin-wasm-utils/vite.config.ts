import { defineConfig } from 'vite';
import { ViteRsw } from 'vite-plugin-rsw';
import { viteStaticCopy } from 'vite-plugin-static-copy';
import dts from 'vite-plugin-dts';

export default defineConfig({
    plugins: [
        ViteRsw(),
        viteStaticCopy({
            targets: [
                {
                    src: './bobbin-wasm-utils/pkg/bobbin_wasm_utils_bg.wasm',
                    dest: '.',
                },
                {
                    src: './bobbin-wasm-utils/pkg/bobbin_wasm_utils.d.ts',
                    dest: '.',
                },
            ],
        }),
        dts(),
    ],
    build: {
        sourcemap: true,
        minify: false,
        lib: {
            formats: ['es'],
            entry: 'src/index.ts',
        },
    },
});
