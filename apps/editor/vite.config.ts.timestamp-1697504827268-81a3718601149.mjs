// vite.config.ts
import { defineConfig } from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite@4.4.7_@types+node@18.17.0/node_modules/vite/dist/node/index.js";
import solid from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-solid@2.7.0_solid-js@1.7.12_vite@4.4.7/node_modules/vite-plugin-solid/dist/esm/index.mjs";
import inspect from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-inspect@0.7.14_rollup@3.26.3_vite@4.4.7/node_modules/vite-plugin-inspect/dist/index.mjs";
import solidDevtoolsPlugin from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/solid-devtools@0.27.7_solid-js@1.7.12_vite@4.4.7/node_modules/solid-devtools/dist/vite.js";
import child_process from "child_process";
import { fileURLToPath, URL } from "node:url";
var __vite_injected_original_import_meta_url = "file:///Users/connormeehan/projects/personal/bearbroidery/apps/editor/vite.config.ts";
var commitHash = child_process.execSync("git rev-parse --short HEAD").toString().trim();
var vite_config_default = defineConfig(({ mode }) => {
  const plugins = [inspect(), solid()];
  if (mode === "development") {
    plugins.push(solidDevtoolsPlugin({ autoname: true }));
  }
  return {
    define: {
      __COMMIT_HASH__: `"${commitHash}"`
    },
    plugins,
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url)),
        src: fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url))
      }
    },
    optimizeDeps: {
      include: ["@kobalte/core"]
    },
    build: {
      emptyOutDir: true,
      minify: true
    }
  };
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvY29ubm9ybWVlaGFuL3Byb2plY3RzL3BlcnNvbmFsL2JlYXJicm9pZGVyeS9hcHBzL2VkaXRvclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL1VzZXJzL2Nvbm5vcm1lZWhhbi9wcm9qZWN0cy9wZXJzb25hbC9iZWFyYnJvaWRlcnkvYXBwcy9lZGl0b3Ivdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL1VzZXJzL2Nvbm5vcm1lZWhhbi9wcm9qZWN0cy9wZXJzb25hbC9iZWFyYnJvaWRlcnkvYXBwcy9lZGl0b3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcsIHR5cGUgUGx1Z2luIH0gZnJvbSAndml0ZSc7XG5pbXBvcnQgc29saWQgZnJvbSAndml0ZS1wbHVnaW4tc29saWQnO1xuaW1wb3J0IGluc3BlY3QgZnJvbSAndml0ZS1wbHVnaW4taW5zcGVjdCc7XG5pbXBvcnQgc29saWREZXZ0b29sc1BsdWdpbiBmcm9tICdzb2xpZC1kZXZ0b29scy92aXRlJztcbmltcG9ydCBjaGlsZF9wcm9jZXNzIGZyb20gJ2NoaWxkX3Byb2Nlc3MnO1xuXG5pbXBvcnQgeyBmaWxlVVJMVG9QYXRoLCBVUkwgfSBmcm9tICdub2RlOnVybCc7XG5cbmNvbnN0IGNvbW1pdEhhc2ggPSBjaGlsZF9wcm9jZXNzXG4gICAgLmV4ZWNTeW5jKCdnaXQgcmV2LXBhcnNlIC0tc2hvcnQgSEVBRCcpXG4gICAgLnRvU3RyaW5nKClcbiAgICAudHJpbSgpO1xuXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoKHsgbW9kZSB9KSA9PiB7XG4gICAgY29uc3QgcGx1Z2lucyA9IFtpbnNwZWN0KCksIHNvbGlkKCldO1xuICAgIGlmIChtb2RlID09PSAnZGV2ZWxvcG1lbnQnKSB7XG4gICAgICAgIHBsdWdpbnMucHVzaChzb2xpZERldnRvb2xzUGx1Z2luKHsgYXV0b25hbWU6IHRydWUgfSkgYXMgUGx1Z2luKTtcbiAgICB9XG5cbiAgICByZXR1cm4ge1xuICAgICAgICBkZWZpbmU6IHtcbiAgICAgICAgICAgIF9fQ09NTUlUX0hBU0hfXzogYFwiJHtjb21taXRIYXNofVwiYCxcbiAgICAgICAgfSxcbiAgICAgICAgcGx1Z2lucyxcbiAgICAgICAgcmVzb2x2ZToge1xuICAgICAgICAgICAgYWxpYXM6IHtcbiAgICAgICAgICAgICAgICAnQCc6IGZpbGVVUkxUb1BhdGgobmV3IFVSTCgnLi9zcmMnLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgICAgICAgICAgICBzcmM6IGZpbGVVUkxUb1BhdGgobmV3IFVSTCgnLi9zcmMnLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgICAgICAgIH0sXG4gICAgICAgIH0sXG4gICAgICAgIG9wdGltaXplRGVwczoge1xuICAgICAgICAgICAgaW5jbHVkZTogWydAa29iYWx0ZS9jb3JlJ10sXG4gICAgICAgIH0sXG4gICAgICAgIGJ1aWxkOiB7XG4gICAgICAgICAgICBlbXB0eU91dERpcjogdHJ1ZSxcbiAgICAgICAgICAgIG1pbmlmeTogdHJ1ZSxcbiAgICAgICAgfSxcbiAgICB9O1xufSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQTRXLFNBQVMsb0JBQWlDO0FBQ3RaLE9BQU8sV0FBVztBQUNsQixPQUFPLGFBQWE7QUFDcEIsT0FBTyx5QkFBeUI7QUFDaEMsT0FBTyxtQkFBbUI7QUFFMUIsU0FBUyxlQUFlLFdBQVc7QUFOaU0sSUFBTSwyQ0FBMkM7QUFRclIsSUFBTSxhQUFhLGNBQ2QsU0FBUyw0QkFBNEIsRUFDckMsU0FBUyxFQUNULEtBQUs7QUFFVixJQUFPLHNCQUFRLGFBQWEsQ0FBQyxFQUFFLEtBQUssTUFBTTtBQUN0QyxRQUFNLFVBQVUsQ0FBQyxRQUFRLEdBQUcsTUFBTSxDQUFDO0FBQ25DLE1BQUksU0FBUyxlQUFlO0FBQ3hCLFlBQVEsS0FBSyxvQkFBb0IsRUFBRSxVQUFVLEtBQUssQ0FBQyxDQUFXO0FBQUEsRUFDbEU7QUFFQSxTQUFPO0FBQUEsSUFDSCxRQUFRO0FBQUEsTUFDSixpQkFBaUIsSUFBSSxVQUFVO0FBQUEsSUFDbkM7QUFBQSxJQUNBO0FBQUEsSUFDQSxTQUFTO0FBQUEsTUFDTCxPQUFPO0FBQUEsUUFDSCxLQUFLLGNBQWMsSUFBSSxJQUFJLFNBQVMsd0NBQWUsQ0FBQztBQUFBLFFBQ3BELEtBQUssY0FBYyxJQUFJLElBQUksU0FBUyx3Q0FBZSxDQUFDO0FBQUEsTUFDeEQ7QUFBQSxJQUNKO0FBQUEsSUFDQSxjQUFjO0FBQUEsTUFDVixTQUFTLENBQUMsZUFBZTtBQUFBLElBQzdCO0FBQUEsSUFDQSxPQUFPO0FBQUEsTUFDSCxhQUFhO0FBQUEsTUFDYixRQUFRO0FBQUEsSUFDWjtBQUFBLEVBQ0o7QUFDSixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
