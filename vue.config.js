const { defineConfig } = require("@vue/cli-service");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = defineConfig({
  transpileDependencies: true,
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
