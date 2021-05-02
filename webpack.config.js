const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const webpack = require("webpack");
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {

    mode: "development",
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "final.js",
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '.')
        }),
    ],
};