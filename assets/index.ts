import rust_wasm_init  from "./vendors";
import { lex, parse } from "./vendors";

rust_wasm_init("/build/")

// const calcul = "(2 + 3 * 4) / (-5e-1 + 6) - 7";
const calcul = "(2 + (4 - 3) * 5) / (9 - 6)".replace(/ /g, "");

console.log(JSON.parse(lex(calcul, true)));
console.log(JSON.parse(parse(calcul, false)));
