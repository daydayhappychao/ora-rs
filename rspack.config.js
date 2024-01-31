/** @type {import('@rspack/cli').Configuration} */
const config = {
  context: __dirname,
  entry: {
    main: "./index.ts",
  },
  output: {
    path: ".",
    filename: "bundle.js",
    libraryTarget: "commonjs",
  },
  mode: "development",
  target: "node10",
  devtool: "source-map",
  externals: ["./binding"],
  externalsType: "commonjs",
  module: {
    rules: [
      {
        test: /\.[t|j]s$/,
        loader: "builtin:swc-loader",
        options: {
          jsc: {
            parser: {
              syntax: "typescript",
            },
          },
        },
        type: "javascript/auto",
      },
    ],
  },
};
module.exports = config;
