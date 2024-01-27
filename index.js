import * as wasm from "./pkg/calculator";

const calcul = "(2 + 3 * 4) / (-5e-1 + 6) - 7";

console.log(JSON.parse(wasm.lex(calcul, true)));
console.log(JSON.parse(wasm.parse(calcul, true)));
