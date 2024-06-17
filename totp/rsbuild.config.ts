import { defineConfig } from '@rsbuild/core';
import { pluginSvelte } from '@rsbuild/plugin-svelte';

// https://rsbuild.dev/config/
export default defineConfig(async () => ({
  plugins: [pluginSvelte()],
  
  // Rsbuild options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Rsbuild from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell Rsbuild to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  html: {
    template: "index.html",
  },
}));
