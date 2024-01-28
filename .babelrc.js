module.exports = (api) => {
    api.cache(process.env.NODE_ENV === 'prod');

    return {
        presets: [
            ["@babel/preset-env", { "useBuiltIns": "entry", "corejs": 3 }],
            "@babel/preset-typescript",
        ],
    };
};
