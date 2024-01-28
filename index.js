import * as wasm from "./pkg/calculator";

// const calcul = "(2 + 3 * 4) / (-5e-1 + 6) - 7";
const calcul = "(2 + 4 - 3 * 5) / (9 - 6)";

console.log(JSON.parse(wasm.lex(calcul, true)));
console.log(JSON.parse(wasm.parse(calcul, false)));
