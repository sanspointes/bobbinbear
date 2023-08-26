// vite.config.ts
import { defineConfig } from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite@4.4.7_@types+node@18.17.0/node_modules/vite/dist/node/index.js";
import solid from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-solid@2.7.0_solid-js@1.7.11_vite@4.4.7/node_modules/vite-plugin-solid/dist/esm/index.mjs";
import inspect from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/vite-plugin-inspect@0.7.14_rollup@3.26.3_vite@4.4.7/node_modules/vite-plugin-inspect/dist/index.mjs";
import solidDevtoolsPlugin from "file:///Users/connormeehan/projects/personal/bearbroidery/node_modules/.pnpm/solid-devtools@0.27.7_solid-js@1.7.11_vite@4.4.7/node_modules/solid-devtools/dist/vite.js";
import child_process from "child_process";
var commitHash = child_process.execSync("git rev-parse --short HEAD").toString().trim();
var vite_config_default = defineConfig(({ mode }) => {
  const plugins = [
    inspect(),
    solid()
  ];
  if (mode === "development") {
    plugins.push(solidDevtoolsPlugin({ autoname: true }));
  }
  return {
    define: {
      "__COMMIT_HASH__": `"${commitHash}"`
    },
    plugins,
    build: {
      emptyOutDir: true,
      minify: true
    }
  };
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvY29ubm9ybWVlaGFuL3Byb2plY3RzL3BlcnNvbmFsL2JlYXJicm9pZGVyeS9hcHBzL2VkaXRvclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL1VzZXJzL2Nvbm5vcm1lZWhhbi9wcm9qZWN0cy9wZXJzb25hbC9iZWFyYnJvaWRlcnkvYXBwcy9lZGl0b3Ivdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL1VzZXJzL2Nvbm5vcm1lZWhhbi9wcm9qZWN0cy9wZXJzb25hbC9iZWFyYnJvaWRlcnkvYXBwcy9lZGl0b3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcsIHR5cGUgUGx1Z2luIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCBzb2xpZCBmcm9tIFwidml0ZS1wbHVnaW4tc29saWRcIjtcbmltcG9ydCBpbnNwZWN0IGZyb20gXCJ2aXRlLXBsdWdpbi1pbnNwZWN0XCI7XG5pbXBvcnQgc29saWREZXZ0b29sc1BsdWdpbiBmcm9tIFwic29saWQtZGV2dG9vbHMvdml0ZVwiO1xuaW1wb3J0IGNoaWxkX3Byb2Nlc3MgZnJvbSBcImNoaWxkX3Byb2Nlc3NcIjtcblxuY29uc3QgY29tbWl0SGFzaCA9IGNoaWxkX3Byb2Nlc3MuZXhlY1N5bmMoXCJnaXQgcmV2LXBhcnNlIC0tc2hvcnQgSEVBRFwiKVxuICAudG9TdHJpbmcoKS50cmltKCk7XG5cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZygoeyBtb2RlIH0pID0+IHtcbiAgY29uc3QgcGx1Z2lucyA9IFtcbiAgICBpbnNwZWN0KCksXG4gICAgc29saWQoKSxcbiAgXTtcbiAgaWYgKG1vZGUgPT09IFwiZGV2ZWxvcG1lbnRcIikge1xuICAgIHBsdWdpbnMucHVzaChzb2xpZERldnRvb2xzUGx1Z2luKHsgYXV0b25hbWU6IHRydWUgfSkgYXMgUGx1Z2luKTtcbiAgfVxuXG4gIHJldHVybiB7XG4gICAgZGVmaW5lOiB7XG4gICAgICBcIl9fQ09NTUlUX0hBU0hfX1wiOiBgXCIke2NvbW1pdEhhc2h9XCJgLFxuICAgIH0sXG4gICAgcGx1Z2lucyxcbiAgICBidWlsZDoge1xuICAgICAgZW1wdHlPdXREaXI6IHRydWUsXG4gICAgICBtaW5pZnk6IHRydWUsXG4gICAgfSxcbiAgfTtcbn0pO1xuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUE0VyxTQUFTLG9CQUFpQztBQUN0WixPQUFPLFdBQVc7QUFDbEIsT0FBTyxhQUFhO0FBQ3BCLE9BQU8seUJBQXlCO0FBQ2hDLE9BQU8sbUJBQW1CO0FBRTFCLElBQU0sYUFBYSxjQUFjLFNBQVMsNEJBQTRCLEVBQ25FLFNBQVMsRUFBRSxLQUFLO0FBRW5CLElBQU8sc0JBQVEsYUFBYSxDQUFDLEVBQUUsS0FBSyxNQUFNO0FBQ3hDLFFBQU0sVUFBVTtBQUFBLElBQ2QsUUFBUTtBQUFBLElBQ1IsTUFBTTtBQUFBLEVBQ1I7QUFDQSxNQUFJLFNBQVMsZUFBZTtBQUMxQixZQUFRLEtBQUssb0JBQW9CLEVBQUUsVUFBVSxLQUFLLENBQUMsQ0FBVztBQUFBLEVBQ2hFO0FBRUEsU0FBTztBQUFBLElBQ0wsUUFBUTtBQUFBLE1BQ04sbUJBQW1CLElBQUksVUFBVTtBQUFBLElBQ25DO0FBQUEsSUFDQTtBQUFBLElBQ0EsT0FBTztBQUFBLE1BQ0wsYUFBYTtBQUFBLE1BQ2IsUUFBUTtBQUFBLElBQ1Y7QUFBQSxFQUNGO0FBQ0YsQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
