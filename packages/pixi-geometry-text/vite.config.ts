import { BuildOptions, defineConfig } from 'vite';
import noBundlePlugin from 'vite-plugin-no-bundle';
import dts from 'vite-plugin-dts';
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig(({ mode }) => {
    const plugins =
        mode !== 'playground'
            ? [dts(), noBundlePlugin({ copy: '**/*.css' }), ViteRsw()]
            : [ViteRsw()];
    const build: BuildOptions | undefined =
        mode !== 'playground'
            ? {
                  sourcemap: true,
                  lib: {
                      formats: ['es'],
                      entry: 'src/index.ts',
                  },
              }
            : undefined;
    return {
        build,
        plugins,
    };
});
