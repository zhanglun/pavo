// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  onwarn: (warning, handler) => {
    // suppress warnings on `vite dev` and `vite build`; but even without this, things still work
    if (warning.code.indexOf("a11y_") === 0) return;
    handler(warning);
  },
  kit: {
    adapter: adapter(),
    vite: {
      build: {
        rollupOptions: {
          input: {
            app: "app.html",
            underLayer: "underlayer.html",
          },
        },
      },
    },
  },
};

export default config;
