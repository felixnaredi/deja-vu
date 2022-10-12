const { defineConfig } = require("@vue/cli-service");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = defineConfig({
  transpileDependencies: true,
  publicPath: process.env.NODE_ENV == "production" ? "/deja-vu" : "",
  pages: {
    index: {
      entry: path.resolve(__dirname, "src", "pages", "home", "main.ts"),
      template: path.resolve(__dirname, "public", "index.html"),
      filename: "index.html",
    },
    history: {
      entry: path.resolve(__dirname, "src", "pages", "history", "main.ts"),
      template: path.resolve(__dirname, "public", "index.html"),
      filename: "history/index.html",
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
