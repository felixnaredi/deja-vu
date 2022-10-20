import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import mpa from "vite-plugin-mpa";
import { fileURLToPath } from "url";

// https://vitejs.dev/config/
export default defineConfig(({ mode, command }) => {
  const baseURL =
    mode == "production" || command == "build" ? "/deja-vu/" : "/";
  return {
    base: baseURL,
    plugins: [
      vue(),
      wasm(),
      mpa.default({
        open: baseURL,
      }),
    ],
    build: {
      target: "esnext",
      rollupOptions: {
        input: {
          index: fileURLToPath(
            new URL("/src/pages/index.html", import.meta.url)
          ),
          gameOver: fileURLToPath(
            new URL("/src/pages/game-over/index.html", import.meta.url)
          ),
        },
      },
    },
    resolve: {
      alias: [
        {
          find: /^@\//,
          replacement: fileURLToPath(new URL("./src/", import.meta.url)),
        },
        {
          find: "deja-vu-wasm",
          replacement: fileURLToPath(
            new URL("./src/wasm/pkg", import.meta.url)
          ),
        },
      ],
    },
  };
});
