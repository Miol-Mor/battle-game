const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
    publicPath: '/',
  },
  plugins: [
    new webpack.DefinePlugin({
      'process.env.WS_ADDRESS': JSON.stringify(process.env.WS_ADDRESS),
      'process.env.WS_PROTOCOL': JSON.stringify(process.env.WS_PROTOCOL),
      'process.env.WS_PORT': JSON.stringify(process.env.WS_PORT),
    })
  ],
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    publicPath: '/',
    hot: true,
    host: '0.0.0.0',
    port: 8080,
    historyApiFallback: true,
    disableHostCheck: true,  // Not recommended unless necessary
  },
};
