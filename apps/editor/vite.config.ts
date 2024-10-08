import { defineConfig, type Plugin } from 'vite';
import solid from 'vite-plugin-solid';
import inspect from 'vite-plugin-inspect';
import solidDevtoolsPlugin from 'solid-devtools/vite';
import child_process from 'child_process';

import { fileURLToPath, URL } from 'node:url';

const commitHash = child_process
    .execSync('git rev-parse --short HEAD')
    .toString()
    .trim();

export default defineConfig(({ mode }) => {
    const plugins = [inspect(), solid()];
    if (mode === 'development') {
        plugins.push(solidDevtoolsPlugin({ autoname: true }) as Plugin);
    }

    return {
        define: {
            __COMMIT_HASH__: `"${commitHash}"`,
        },
        plugins,
        resolve: {
            alias: {
                '@': fileURLToPath(new URL('./src', import.meta.url)),
                src: fileURLToPath(new URL('./src', import.meta.url)),
            },
        },
        optimizeDeps: {
            include: ['@kobalte/core'],
        },
        build: {
            emptyOutDir: true,
            minify: true,
        },
    };
});
