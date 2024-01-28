"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkcalculator"] = self["webpackChunkcalculator"] || []).push([["index_js"],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_calculator__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pkg/calculator */ \"./pkg/calculator.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_pkg_calculator__WEBPACK_IMPORTED_MODULE_0__]);\n_pkg_calculator__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n// const calcul = \"(2 + 3 * 4) / (-5e-1 + 6) - 7\";\nconst calcul = \"(2 + 4 - 3 * 5) / (9 - 6)\";\n\nconsole.log(JSON.parse(_pkg_calculator__WEBPACK_IMPORTED_MODULE_0__.lex(calcul, true)));\nconsole.log(JSON.parse(_pkg_calculator__WEBPACK_IMPORTED_MODULE_0__.parse(calcul, false)));\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://calculator/./index.js?");

/***/ }),

/***/ "./pkg/calculator.js":
/*!***************************!*\
  !*** ./pkg/calculator.js ***!
  \***************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _calculator_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   lex: () => (/* reexport safe */ _calculator_bg_js__WEBPACK_IMPORTED_MODULE_0__.lex),\n/* harmony export */   parse: () => (/* reexport safe */ _calculator_bg_js__WEBPACK_IMPORTED_MODULE_0__.parse)\n/* harmony export */ });\n/* harmony import */ var _calculator_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./calculator_bg.wasm */ \"./pkg/calculator_bg.wasm\");\n/* harmony import */ var _calculator_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./calculator_bg.js */ \"./pkg/calculator_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_calculator_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_calculator_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_calculator_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_calculator_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://calculator/./pkg/calculator.js?");

/***/ }),

/***/ "./pkg/calculator_bg.js":
/*!******************************!*\
  !*** ./pkg/calculator_bg.js ***!
  \******************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   lex: () => (/* binding */ lex),\n/* harmony export */   parse: () => (/* binding */ parse)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} text\n* @param {boolean} with_position\n* @returns {string}\n*/\nfunction lex(text, with_position) {\n    let deferred2_0;\n    let deferred2_1;\n    try {\n        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        wasm.lex(retptr, ptr0, len0, with_position);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        deferred2_0 = r0;\n        deferred2_1 = r1;\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        wasm.__wbindgen_add_to_stack_pointer(16);\n        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);\n    }\n}\n\n/**\n* @param {string} text\n* @param {boolean} with_position\n* @returns {string}\n*/\nfunction parse(text, with_position) {\n    let deferred2_0;\n    let deferred2_1;\n    try {\n        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        wasm.parse(retptr, ptr0, len0, with_position);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        deferred2_0 = r0;\n        deferred2_1 = r1;\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        wasm.__wbindgen_add_to_stack_pointer(16);\n        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);\n    }\n}\n\n\n\n//# sourceURL=webpack://calculator/./pkg/calculator_bg.js?");

/***/ }),

/***/ "./pkg/calculator_bg.wasm":
/*!********************************!*\
  !*** ./pkg/calculator_bg.wasm ***!
  \********************************/
/***/ ((module, exports, __webpack_require__) => {

eval("module.exports = __webpack_require__.v(exports, module.id, \"7cd833b5b2bb0fd5edde\");\n\n//# sourceURL=webpack://calculator/./pkg/calculator_bg.wasm?");

/***/ })

}]);