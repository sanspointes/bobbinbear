import { fileURLToPath, URL } from "url";
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';
import tsconfigPaths from 'vite-tsconfig-paths';
import devtools from 'solid-devtools/vite';

export default defineConfig({
    plugins: [solid(), wasm(), tsconfigPaths(), devtools()],
    resolve: {
        alias: [
            { find: '~', replacement: fileURLToPath(new URL('./src', import.meta.url)) },
        ]
    }
});
