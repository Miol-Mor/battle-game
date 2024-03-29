const path = require('path');
const Dotenv = require('dotenv-webpack');

module.exports = {
  entry: './src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    contentBase: path.resolve(__dirname, '.'),
    hot: true,
    host: '0.0.0.0',
    disableHostCheck: true,
  },
  plugins: [
    new Dotenv(),
  ],
};
