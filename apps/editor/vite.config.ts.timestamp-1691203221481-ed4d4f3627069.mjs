// vite.config.ts
import { defineConfig } from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite@4.4.7_@types+node@18.17.0/node_modules/vite/dist/node/index.js";
import solid from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-solid@2.7.0_solid-js@1.7.8_vite@4.4.7/node_modules/vite-plugin-solid/dist/esm/index.mjs";
import dts from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-dts@3.3.1_typescript@5.0.4_vite@4.4.7/node_modules/vite-plugin-dts/dist/index.mjs";
import inspect from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-inspect@0.7.14_rollup@3.26.3_vite@4.4.7/node_modules/vite-plugin-inspect/dist/index.mjs";
import solidDevtoolsPlugin from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/solid-devtools@0.27.4_solid-js@1.7.8_vite@4.4.7/node_modules/solid-devtools/dist/vite.js";
var vite_config_default = defineConfig(({ mode }) => {
  const plugins = [
    dts({
      insertTypesEntry: true
    }),
    inspect(),
    solid()
  ];
  if (mode === "development") {
    plugins.push(solidDevtoolsPlugin({ autoname: true }));
  }
  return {
    plugins,
    build: {
      emptyOutDir: false,
      lib: {
        entry: "./src/index.ts",
        formats: ["es", "cjs", "umd"],
        fileName: (format) => `editor.${format}.js`,
        name: "editor"
      },
      minify: false,
      rollupOptions: {
        external: [
          "solid-js",
          "solid-js/web",
          "solid-js/store"
        ]
      }
    }
  };
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvY29ubm9ybWVlaGFuL3Byb2plY3RzL3BlcnNvbmFsL2JlYXJicm9pZGVyeS9wYWNrYWdlcy9lZGl0b3JcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9Vc2Vycy9jb25ub3JtZWVoYW4vcHJvamVjdHMvcGVyc29uYWwvYmVhcmJyb2lkZXJ5L3BhY2thZ2VzL2VkaXRvci92aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vVXNlcnMvY29ubm9ybWVlaGFuL3Byb2plY3RzL3BlcnNvbmFsL2JlYXJicm9pZGVyeS9wYWNrYWdlcy9lZGl0b3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcsIHR5cGUgUGx1Z2luIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCBzb2xpZCBmcm9tICd2aXRlLXBsdWdpbi1zb2xpZCdcbmltcG9ydCBkdHMgZnJvbSBcInZpdGUtcGx1Z2luLWR0c1wiO1xuaW1wb3J0IGluc3BlY3QgZnJvbSBcInZpdGUtcGx1Z2luLWluc3BlY3RcIjtcbmltcG9ydCBzb2xpZERldnRvb2xzUGx1Z2luIGZyb20gXCJzb2xpZC1kZXZ0b29scy92aXRlXCI7XG5cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZygoe21vZGV9KSAgPT4ge1xuICBjb25zdCBwbHVnaW5zID0gW1xuICAgIGR0cyh7XG4gICAgICBpbnNlcnRUeXBlc0VudHJ5OiB0cnVlLFxuICAgIH0pLFxuICAgIGluc3BlY3QoKSxcbiAgICBzb2xpZCgpLFxuICBdO1xuICBpZiAobW9kZSA9PT0gXCJkZXZlbG9wbWVudFwiKSB7XG4gICAgcGx1Z2lucy5wdXNoKHNvbGlkRGV2dG9vbHNQbHVnaW4oeyBhdXRvbmFtZTogdHJ1ZSB9KSBhcyBQbHVnaW4pO1xuICB9XG5cbiAgcmV0dXJuIHtcbiAgICBwbHVnaW5zLFxuICAgIGJ1aWxkOiB7XG4gICAgICBlbXB0eU91dERpcjogZmFsc2UsXG4gICAgICBsaWI6IHtcbiAgICAgICAgZW50cnk6ICcuL3NyYy9pbmRleC50cycsXG4gICAgICAgIGZvcm1hdHM6IFsnZXMnLCAnY2pzJywgJ3VtZCddLFxuICAgICAgICBmaWxlTmFtZTogKGZvcm1hdCkgPT4gYGVkaXRvci4ke2Zvcm1hdH0uanNgLFxuICAgICAgICBuYW1lOiAnZWRpdG9yJyxcbiAgICAgIH0sXG4gICAgICBtaW5pZnk6IGZhbHNlLFxuICAgICAgcm9sbHVwT3B0aW9uczoge1xuICAgICAgICBleHRlcm5hbDogW1xuICAgICAgICAgICdzb2xpZC1qcycsXG4gICAgICAgICAgJ3NvbGlkLWpzL3dlYicsXG4gICAgICAgICAgJ3NvbGlkLWpzL3N0b3JlJyxcbiAgICAgICAgXVxuICAgICAgfSxcbiAgICB9LFxuICB9XG59KVxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUF3WCxTQUFTLG9CQUFpQztBQUNsYSxPQUFPLFdBQVc7QUFDbEIsT0FBTyxTQUFTO0FBQ2hCLE9BQU8sYUFBYTtBQUNwQixPQUFPLHlCQUF5QjtBQUVoQyxJQUFPLHNCQUFRLGFBQWEsQ0FBQyxFQUFDLEtBQUksTUFBTztBQUN2QyxRQUFNLFVBQVU7QUFBQSxJQUNkLElBQUk7QUFBQSxNQUNGLGtCQUFrQjtBQUFBLElBQ3BCLENBQUM7QUFBQSxJQUNELFFBQVE7QUFBQSxJQUNSLE1BQU07QUFBQSxFQUNSO0FBQ0EsTUFBSSxTQUFTLGVBQWU7QUFDMUIsWUFBUSxLQUFLLG9CQUFvQixFQUFFLFVBQVUsS0FBSyxDQUFDLENBQVc7QUFBQSxFQUNoRTtBQUVBLFNBQU87QUFBQSxJQUNMO0FBQUEsSUFDQSxPQUFPO0FBQUEsTUFDTCxhQUFhO0FBQUEsTUFDYixLQUFLO0FBQUEsUUFDSCxPQUFPO0FBQUEsUUFDUCxTQUFTLENBQUMsTUFBTSxPQUFPLEtBQUs7QUFBQSxRQUM1QixVQUFVLENBQUMsV0FBVyxVQUFVLE1BQU07QUFBQSxRQUN0QyxNQUFNO0FBQUEsTUFDUjtBQUFBLE1BQ0EsUUFBUTtBQUFBLE1BQ1IsZUFBZTtBQUFBLFFBQ2IsVUFBVTtBQUFBLFVBQ1I7QUFBQSxVQUNBO0FBQUEsVUFDQTtBQUFBLFFBQ0Y7QUFBQSxNQUNGO0FBQUEsSUFDRjtBQUFBLEVBQ0Y7QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
