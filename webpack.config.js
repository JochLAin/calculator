const Encore = require('@symfony/webpack-encore');
const path = require('path');
const pkg = require('./package.json');

// Manually configure the runtime environment if not already configured yet by the "encore" command.
// It's useful when you use tools that rely on webpack.config.js file.
if (!Encore.isRuntimeEnvironmentConfigured()) {
  Encore.configureRuntimeEnvironment(process.env.NODE_ENV || 'dev');
}

Encore
  .cleanupOutputBeforeBuild()
  .splitEntryChunks()

  .enableBuildNotifications(process.env.ENABLE_BUILD_NOTIFICATION || false)
  .enableIntegrityHashes(Encore.isProduction())
  .enableSingleRuntimeChunk()
  .enableSourceMaps(!Encore.isProduction())
  .enableVersioning(Encore.isProduction())

  .enableSassLoader()
  .setOutputPath('public/build/')
  .setPublicPath('/build')

  .configureLoaderRule('javascript', (loader) => {
    loader.test = /\.([em]?[jt]sx?)$/;
  })

  .addAliases(Object.entries(pkg._moduleAliases).reduce((accu, [key, filename]) => {
    return Object.assign(accu, { [key]: path.resolve(process.cwd(), filename) });
  }, {}))
;

module.exports = Promise.resolve().then(() => {
  if (Encore.isProduction()) {
    // Do production stuff
  } else {
    const { PORT_WEBPACK } = process.env;

    Encore
      .configureDevServerOptions((options) => {
        Object.assign(options, {
          allowedHosts: 'all',
          client: { webSocketURL: `ws://127.0.0.1:${PORT_WEBPACK}/ws`, },
          compress: true,
          host: '0.0.0.0',
          port: PORT_WEBPACK,
          hot: true,
          liveReload: true,
        });
      })
    ;
  }
}).then(() => {
  Encore.addEntry('bootstrap', path.resolve(__dirname, 'assets/bootstrap.ts'));
}).then(() => {
  return Object.assign(Encore.getWebpackConfig(), {
    experiments: {
      asyncWebAssembly: true,
    }
  });
});
