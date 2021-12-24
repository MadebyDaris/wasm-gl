const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin"); // installed via npm
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: {
    main: path.resolve(__dirname, "./js/lib.js"),
    // page: path.resolve(__dirname, "src/webrs-header.js"),
    // detail: path.resolve(__dirname, "src/detail-view.js"),
  },
  devtool: "inline-source-map",
  output: {
    filename: "[name].js",
    chunkFilename: "[name].js",
    path: path.resolve(__dirname, "dist"),
  },

  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."), // Define where the root of the rust code is located (where the cargo.toml file is located)
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: "**/*.html",
          to: ".",
          context: ".",
        },
        // {
        //   from: "**/*.js",
        //   to: ".",
        //   context: "js",
        // },
      ],
    }),
    new CleanWebpackPlugin(),
  ],
  mode: 'development',
  experiments: {
      asyncWebAssembly: true,
  }
}
