const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  mode: "development",
  output: {
    path: path.resolve(__dirname, "public/build"),
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true,
  }
};
