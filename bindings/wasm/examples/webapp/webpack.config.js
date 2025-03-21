const Dotenv = require('dotenv-webpack');
const path = require('path');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require('webpack');

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
      ],
    }),
    new Dotenv({
      path: '../.env'
    })
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
};
