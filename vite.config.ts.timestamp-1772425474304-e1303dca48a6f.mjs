// vite.config.ts
import { defineConfig } from "file:///Users/swiftrans/project/Armor/armor/node_modules/vite/dist/node/index.js";
import "file:///Users/swiftrans/project/Armor/armor/node_modules/@sveltejs/vite-plugin-svelte/src/index.js";
import { sveltekit } from "file:///Users/swiftrans/project/Armor/armor/node_modules/@sveltejs/kit/src/exports/vite/index.js";
import tailwindcss from "file:///Users/swiftrans/project/Armor/armor/node_modules/@tailwindcss/vite/dist/index.mjs";
import franken from "file:///Users/swiftrans/project/Armor/armor/node_modules/franken-ui/dist/plugin-vite.js";
var vite_config_default = defineConfig({
  plugins: [
    franken({ preflight: false }),
    tailwindcss(),
    sveltekit()
  ],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome100", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvc3dpZnRyYW5zL3Byb2plY3QvQXJtb3IvYXJtb3JcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9Vc2Vycy9zd2lmdHJhbnMvcHJvamVjdC9Bcm1vci9hcm1vci92aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vVXNlcnMvc3dpZnRyYW5zL3Byb2plY3QvQXJtb3IvYXJtb3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlJztcbmltcG9ydCB7IHN2ZWx0ZSB9IGZyb20gJ0BzdmVsdGVqcy92aXRlLXBsdWdpbi1zdmVsdGUnO1xuaW1wb3J0IHsgc3ZlbHRla2l0IH0gZnJvbSAnQHN2ZWx0ZWpzL2tpdC92aXRlJztcbmltcG9ydCB0YWlsd2luZGNzcyBmcm9tICdAdGFpbHdpbmRjc3Mvdml0ZSc7XG5pbXBvcnQgZnJhbmtlbiBmcm9tICdmcmFua2VuLXVpL3BsdWdpbi12aXRlJztcblxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcbiAgcGx1Z2luczogW1xuICAgIGZyYW5rZW4oeyBwcmVmbGlnaHQ6IGZhbHNlIH0pLFxuICAgIHRhaWx3aW5kY3NzKCksXG4gICAgc3ZlbHRla2l0KClcbiAgXSxcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiA1MTczLFxuICAgIHN0cmljdFBvcnQ6IHRydWUsXG4gICAgd2F0Y2g6IHtcbiAgICAgIGlnbm9yZWQ6IFsnKiovc3JjLXRhdXJpLyoqJ11cbiAgICB9XG4gIH0sXG4gIGVudlByZWZpeDogWydWSVRFXycsICdUQVVSSV8nXSxcbiAgYnVpbGQ6IHtcbiAgICB0YXJnZXQ6IFsnZXMyMDIxJywgJ2Nocm9tZTEwMCcsICdzYWZhcmkxMyddLFxuICAgIG1pbmlmeTogIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHID8gJ2VzYnVpbGQnIDogZmFsc2UsXG4gICAgc291cmNlbWFwOiAhIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHLFxuICB9XG59KTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBOFIsU0FBUyxvQkFBb0I7QUFDM1QsT0FBdUI7QUFDdkIsU0FBUyxpQkFBaUI7QUFDMUIsT0FBTyxpQkFBaUI7QUFDeEIsT0FBTyxhQUFhO0FBRXBCLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQzFCLFNBQVM7QUFBQSxJQUNQLFFBQVEsRUFBRSxXQUFXLE1BQU0sQ0FBQztBQUFBLElBQzVCLFlBQVk7QUFBQSxJQUNaLFVBQVU7QUFBQSxFQUNaO0FBQUEsRUFDQSxhQUFhO0FBQUEsRUFDYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixPQUFPO0FBQUEsTUFDTCxTQUFTLENBQUMsaUJBQWlCO0FBQUEsSUFDN0I7QUFBQSxFQUNGO0FBQUEsRUFDQSxXQUFXLENBQUMsU0FBUyxRQUFRO0FBQUEsRUFDN0IsT0FBTztBQUFBLElBQ0wsUUFBUSxDQUFDLFVBQVUsYUFBYSxVQUFVO0FBQUEsSUFDMUMsUUFBUSxDQUFDLFFBQVEsSUFBSSxjQUFjLFlBQVk7QUFBQSxJQUMvQyxXQUFXLENBQUMsQ0FBQyxRQUFRLElBQUk7QUFBQSxFQUMzQjtBQUNGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
