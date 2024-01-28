import fs from "fs";
import path from "path";

export default function markup(view: string, content: string = '') {
  const { entrypoints: { [view]: { css = [], js = [] } } } = JSON.parse(fs.readFileSync(path.resolve(process.cwd(), 'public/build/entrypoints.json')).toString());

  return `<!DOCTYPE html>
<html lang="fr">
  <head>
    <title>Calcul</title>
    <meta charset="utf-8">
    <meta name="description" content="Calculation with rust engine">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="">
    ${[...css, ...js].map((src: string) => `<link rel="preload" href="${src}">`).join("\n")}
    <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet">
    ${css.map((src: string) => `<link href="${src}" rel="stylesheet">`).join("\n")}
  </head>
  <body>
    ${content}
    ${js.map((src: string) => `<script src="${src}" async defer></script>`).join("\n")}
  </body>
</html>`;
}
