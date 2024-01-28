import express from "express";
import https from "https";
import path from "path";
import fs from "fs";
import render from "./renderer";
import markup from "./renderer";

const router = express();
https.createServer({
  cert: fs.readFileSync(path.resolve(__dirname, '../docker/front/ssl.crt')),
  key: fs.readFileSync(path.resolve(__dirname, '../docker/front/ssl.key')),
}, router).listen(443, () => {
  console.log('Server started !');
});

router.get('/', (request, response) => {
  response.send(markup('bootstrap', '<my-calcul></my-calcul>'));
});

router.use('/', express.static(path.resolve(process.cwd(), 'public')));
