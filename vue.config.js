const { defineConfig } = require("@vue/cli-service");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = defineConfig({
  transpileDependencies: true,
  publicPath: process.env.NODE_ENV == "production" ? "/deja-vu/" : "/deja-vu/",
  pages: {
    index: {
      entry: path.resolve(__dirname, "src", "pages", "home", "main.ts"),
      template: path.resolve(__dirname, "public", "index.html"),
      filename: "index.html",
    },
    "game-over": {
      entry: path.resolve(__dirname, "src", "pages", "game-over", "main.ts"),
      template: path.resolve(__dirname, "public", "index.html"),
      filename: "game-over/index.html",
    },
  },
  configureWebpack: {
    plugins: [
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "src", "wasm"),
        outDir: path.resolve(__dirname, "dist", "wasm"),
      }),
    ],
    experiments: {
      asyncWebAssembly: true,
    },
  },
});
