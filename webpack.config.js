const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  entry: "./index.js",
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  devServer: {
    open: true,
  },
  experiments: {
    asyncWebAssembly: true,
  },
};
