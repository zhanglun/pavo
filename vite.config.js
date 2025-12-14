import { svelte } from "@sveltejs/vite-plugin-svelte";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

const __dirname = dirname(fileURLToPath(import.meta.url));
console.log("🚀 ~ __dirname:", __dirname)

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "src", "index.html"),
        underLayer: resolve(__dirname, "src", "underlayer.html"),
      },
    },
  },
}));
