const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");

module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8001
    },

    entry: [
      path.resolve(__dirname, "bootstrap.js"),
    ],

    output: {
      path: distPath,
      filename: "index.js",
      webassemblyModuleFilename: "index.wasm"
    },

    plugins: [
      new CleanWebpackPlugin(),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname),
        extraArgs: "--no-typescript",
        outDir: path.resolve(__dirname, "pkg"),
      }),
      new CopyPlugin({
        patterns: [
          {
            from: path.resolve(__dirname, "static"),
            to: path.resolve(__dirname, "dist")
          }
        ]
      }),
    ],
    watch: argv.mode !== "production"
  }
}
