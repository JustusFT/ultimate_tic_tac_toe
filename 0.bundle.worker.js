self["webpackChunk"]([0],{

/***/ "./wasm-bindings/pkg sync recursive":
/*!********************************!*\
  !*** ./wasm-bindings/pkg sync ***!
  \********************************/
/*! no static exports found */
/***/ (function(module, exports) {

function webpackEmptyContext(req) {
	var e = new Error("Cannot find module '" + req + "'");
	e.code = 'MODULE_NOT_FOUND';
	throw e;
}
webpackEmptyContext.keys = function() { return []; };
webpackEmptyContext.resolve = webpackEmptyContext;
module.exports = webpackEmptyContext;
webpackEmptyContext.id = "./wasm-bindings/pkg sync recursive";

/***/ }),

/***/ "./wasm-bindings/pkg/index.js":
/*!************************************!*\
  !*** ./wasm-bindings/pkg/index.js ***!
  \************************************/
/*! exports provided: init, new_game, new_mcts, new_from_fen, get_fen, get_game_state, cpu_move, Piece, Game, MctsTree, ZorbistHasher, __wbg_now_2ed7c83e40d461d6, __wbindgen_json_parse, __wbindgen_string_new, __wbindgen_object_drop_ref, __wbg_self_1801c027cb0e6124, __wbg_crypto_3e91f24788b1203d, __wbg_getRandomValues_7ecea3ecacbb2f9e, __wbg_getRandomValues_f724b5822126eff7, __wbg_require_e89d842e759f0a4c, __wbg_randomFillSync_eae3007264ffc138, __wbindgen_is_undefined, __wbg_error_4bb6c2a97407129a, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "init", function() { return init; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "new_game", function() { return new_game; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "new_mcts", function() { return new_mcts; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "new_from_fen", function() { return new_from_fen; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "get_fen", function() { return get_fen; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "get_game_state", function() { return get_game_state; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "cpu_move", function() { return cpu_move; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Piece", function() { return Piece; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Game", function() { return Game; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "MctsTree", function() { return MctsTree; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ZorbistHasher", function() { return ZorbistHasher; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_now_2ed7c83e40d461d6", function() { return __wbg_now_2ed7c83e40d461d6; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_parse", function() { return __wbindgen_json_parse; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_string_new", function() { return __wbindgen_string_new; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_drop_ref", function() { return __wbindgen_object_drop_ref; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_self_1801c027cb0e6124", function() { return __wbg_self_1801c027cb0e6124; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_crypto_3e91f24788b1203d", function() { return __wbg_crypto_3e91f24788b1203d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getRandomValues_7ecea3ecacbb2f9e", function() { return __wbg_getRandomValues_7ecea3ecacbb2f9e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getRandomValues_f724b5822126eff7", function() { return __wbg_getRandomValues_f724b5822126eff7; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_require_e89d842e759f0a4c", function() { return __wbg_require_e89d842e759f0a4c; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_randomFillSync_eae3007264ffc138", function() { return __wbg_randomFillSync_eae3007264ffc138; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_is_undefined", function() { return __wbindgen_is_undefined; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_error_4bb6c2a97407129a", function() { return __wbg_error_4bb6c2a97407129a; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_59cb74e423758ede", function() { return __wbg_new_59cb74e423758ede; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_stack_558ba5917b466edd", function() { return __wbg_stack_558ba5917b466edd; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return __wbindgen_throw; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_rethrow", function() { return __wbindgen_rethrow; });
/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ "./wasm-bindings/pkg/index_bg.wasm");

/**
*/

function init() {
  _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["init"]();
}
/**
* @returns {Game}
*/

function new_game() {
  const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["new_game"]();
  return Game.__wrap(ret);
}
/**
* @returns {MctsTree}
*/

function new_mcts() {
  const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["new_mcts"]();
  return MctsTree.__wrap(ret);
}
let WASM_VECTOR_LEN = 0;
let cachedTextEncoder = new TextEncoder('utf-8');
const encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function (arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};
let cachegetUint8Memory = null;

function getUint8Memory() {
  if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
    cachegetUint8Memory = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
  }

  return cachegetUint8Memory;
}

function passStringToWasm(arg) {
  if (typeof arg !== 'string') throw new Error('expected a string argument');
  let len = arg.length;

  let ptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"](len);

  const mem = getUint8Memory();
  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }

    ptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"](ptr, len, len = offset + arg.length * 3);
    const view = getUint8Memory().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    if (ret.read != arg.length) throw new Error('failed to pass whole string');
    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}
/**
* @param {string} fen
* @returns {Game}
*/


function new_from_fen(fen) {
  const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["new_from_fen"](passStringToWasm(fen), WASM_VECTOR_LEN);
  return Game.__wrap(ret);
}

function _assertNum(n) {
  if (typeof n !== 'number') throw new Error('expected a number argument');
}

function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }

  return instance.ptr;
}

let cachegetInt32Memory = null;

function getInt32Memory() {
  if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
    cachegetInt32Memory = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
  }

  return cachegetInt32Memory;
}

let cachedTextDecoder = new TextDecoder('utf-8', {
  ignoreBOM: true,
  fatal: true
});

function getStringFromWasm(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}
/**
* @param {Game} game
* @returns {string}
*/


function get_fen(game) {
  const retptr = 8;

  _assertNum(retptr);

  _assertClass(game, Game);

  if (game.ptr === 0) {
    throw new Error('Attempt to use a moved value');
  }

  const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["get_fen"](retptr, game.ptr);
  const memi32 = getInt32Memory();
  const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();

  _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);

  return v0;
}
const heap = new Array(32);
heap.fill(undefined);
heap.push(undefined, null, true, false);

function getObject(idx) {
  return heap[idx];
}

let heap_next = heap.length;

function dropObject(idx) {
  if (idx < 36) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
/**
* @param {Game} game
* @returns {any}
*/


function get_game_state(game) {
  _assertClass(game, Game);

  if (game.ptr === 0) {
    throw new Error('Attempt to use a moved value');
  }

  const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["get_game_state"](game.ptr);
  return takeObject(ret);
}
/**
* @param {Game} game
* @param {MctsTree} tree
*/

function cpu_move(game, tree) {
  _assertClass(game, Game);

  if (game.ptr === 0) {
    throw new Error('Attempt to use a moved value');
  }

  _assertClass(tree, MctsTree);

  if (tree.ptr === 0) {
    throw new Error('Attempt to use a moved value');
  }

  _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["cpu_move"](game.ptr, tree.ptr);
}

function isLikeNone(x) {
  return x === undefined || x === null;
}

const u32CvtShim = new Uint32Array(2);
const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);
let cachegetUint32Memory = null;

function getUint32Memory() {
  if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
    cachegetUint32Memory = new Uint32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
  }

  return cachegetUint32Memory;
}

function logError(e) {
  let error = function () {
    try {
      return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
    } catch (_) {
      return "<failed to stringify thrown value>";
    }
  }();

  console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
  throw e;
}

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  if (typeof heap_next !== 'number') throw new Error('corrupt heap');
  heap[idx] = obj;
  return idx;
}

function handleError(e) {
  _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_exn_store"](addHeapObject(e));
}

function getArrayU8FromWasm(ptr, len) {
  return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

function _assertBoolean(n) {
  if (typeof n !== 'boolean') {
    throw new Error('expected a boolean argument');
  }
}
/**
*/


const Piece = Object.freeze({
  BLANK: 0,
  X: 1,
  O: 2
});
/**
*/

class Game {
  constructor() {
    throw new Error('cannot invoke `new` directly');
  }

  static __wrap(ptr) {
    const obj = Object.create(Game.prototype);
    obj.ptr = ptr;
    return obj;
  }

  free() {
    const ptr = this.ptr;
    this.ptr = 0;

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_game_free"](ptr);
  }
  /**
  * @returns {number}
  */


  get current_board() {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_game_current_board"](this.ptr);

    return ret === 0xFFFFFF ? undefined : ret;
  }
  /**
  * @param {number | undefined} arg0
  */


  set current_board(arg0) {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    if (!isLikeNone(arg0)) {
      _assertNum(arg0);
    }

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_set_game_current_board"](this.ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0);
  }
  /**
  * @returns {number}
  */


  get turn() {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_game_turn"](this.ptr);

    return ret;
  }
  /**
  * @param {number} arg0
  */


  set turn(arg0) {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    _assertNum(arg0);

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_set_game_turn"](this.ptr, arg0);
  }
  /**
  * @returns {number}
  */


  get winner() {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_game_winner"](this.ptr);

    return ret === 3 ? undefined : ret;
  }
  /**
  * @param {number | undefined} arg0
  */


  set winner(arg0) {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    if (!isLikeNone(arg0)) {
      _assertNum(arg0);
    }

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_set_game_winner"](this.ptr, isLikeNone(arg0) ? 3 : arg0);
  }
  /**
  * @returns {BigInt}
  */


  get hash() {
    const retptr = 8;
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(retptr);

    _assertNum(this.ptr);

    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_game_hash"](retptr, this.ptr);

    const memi32 = getInt32Memory();
    u32CvtShim[0] = memi32[retptr / 4 + 0];
    u32CvtShim[1] = memi32[retptr / 4 + 1];
    const n0 = uint64CvtShim[0];
    return n0;
  }
  /**
  * @param {BigInt} arg0
  */


  set hash(arg0) {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    uint64CvtShim[0] = arg0;
    const low0 = u32CvtShim[0];
    const high0 = u32CvtShim[1];

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_set_game_hash"](this.ptr, low0, high0);
  }
  /**
  * @returns {Game}
  */


  static new() {
    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["game_new"]();
    return Game.__wrap(ret);
  }
  /**
  * @param {number} local_board
  * @param {number} cell
  */


  make_move(local_board, cell) {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    _assertNum(local_board);

    _assertNum(cell);

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["game_make_move"](this.ptr, local_board, cell);
  }
  /**
  */


  undo_move() {
    if (this.ptr == 0) throw new Error('Attempt to use a moved value');

    _assertNum(this.ptr);

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["game_undo_move"](this.ptr);
  }

}
/**
*/

class MctsTree {
  constructor() {
    throw new Error('cannot invoke `new` directly');
  }

  static __wrap(ptr) {
    const obj = Object.create(MctsTree.prototype);
    obj.ptr = ptr;
    return obj;
  }

  free() {
    const ptr = this.ptr;
    this.ptr = 0;

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_mctstree_free"](ptr);
  }

}
/**
*/

class ZorbistHasher {
  constructor() {
    throw new Error('cannot invoke `new` directly');
  }

  free() {
    const ptr = this.ptr;
    this.ptr = 0;

    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_zorbisthasher_free"](ptr);
  }

}
const __wbg_now_2ed7c83e40d461d6 = function () {
  try {
    const ret = Date.now();

    _assertNum(ret);

    return ret;
  } catch (e) {
    logError(e);
  }
};
const __wbindgen_json_parse = function (arg0, arg1) {
  const ret = JSON.parse(getStringFromWasm(arg0, arg1));
  return addHeapObject(ret);
};
const __wbindgen_string_new = function (arg0, arg1) {
  const ret = getStringFromWasm(arg0, arg1);
  return addHeapObject(ret);
};
const __wbindgen_object_drop_ref = function (arg0) {
  takeObject(arg0);
};
const __wbg_self_1801c027cb0e6124 = function () {
  try {
    try {
      const ret = self.self;
      return addHeapObject(ret);
    } catch (e) {
      handleError(e);
    }
  } catch (e) {
    logError(e);
  }
};
const __wbg_crypto_3e91f24788b1203d = function (arg0) {
  try {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
  } catch (e) {
    logError(e);
  }
};
const __wbg_getRandomValues_7ecea3ecacbb2f9e = function (arg0) {
  try {
    const ret = getObject(arg0).getRandomValues;
    return addHeapObject(ret);
  } catch (e) {
    logError(e);
  }
};
const __wbg_getRandomValues_f724b5822126eff7 = function (arg0, arg1, arg2) {
  try {
    getObject(arg0).getRandomValues(getArrayU8FromWasm(arg1, arg2));
  } catch (e) {
    logError(e);
  }
};
const __wbg_require_e89d842e759f0a4c = function (arg0, arg1) {
  try {
    const ret = __webpack_require__("./wasm-bindings/pkg sync recursive")(getStringFromWasm(arg0, arg1));

    return addHeapObject(ret);
  } catch (e) {
    logError(e);
  }
};
const __wbg_randomFillSync_eae3007264ffc138 = function (arg0, arg1, arg2) {
  try {
    getObject(arg0).randomFillSync(getArrayU8FromWasm(arg1, arg2));
  } catch (e) {
    logError(e);
  }
};
const __wbindgen_is_undefined = function (arg0) {
  const ret = getObject(arg0) === undefined;

  _assertBoolean(ret);

  return ret;
};
const __wbg_error_4bb6c2a97407129a = function (arg0, arg1) {
  const v0 = getStringFromWasm(arg0, arg1).slice();

  _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](arg0, arg1 * 1);

  try {
    console.error(v0);
  } catch (e) {
    logError(e);
  }
};
const __wbg_new_59cb74e423758ede = function () {
  try {
    const ret = new Error();
    return addHeapObject(ret);
  } catch (e) {
    logError(e);
  }
};
const __wbg_stack_558ba5917b466edd = function (arg0, arg1) {
  try {
    const ret = getObject(arg1).stack;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
  } catch (e) {
    logError(e);
  }
};
const __wbindgen_throw = function (arg0, arg1) {
  throw new Error(getStringFromWasm(arg0, arg1));
};
const __wbindgen_rethrow = function (arg0) {
  throw takeObject(arg0);
};

_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_start"]();

/***/ }),

/***/ "./wasm-bindings/pkg/index_bg.wasm":
/*!*****************************************!*\
  !*** ./wasm-bindings/pkg/index_bg.wasm ***!
  \*****************************************/
/*! exports provided: memory, __rustc_debug_gdb_scripts_section__, init, new_game, new_mcts, new_from_fen, get_fen, get_game_state, cpu_move, __wbg_zorbisthasher_free, __wbg_game_free, __wbg_get_game_current_board, __wbg_set_game_current_board, __wbg_get_game_turn, __wbg_set_game_turn, __wbg_get_game_winner, __wbg_set_game_winner, __wbg_get_game_hash, __wbg_set_game_hash, game_new, game_make_move, game_undo_move, __wbg_mctstree_free, __wbindgen_exn_store, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.i];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name != "__webpack_init__") exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./index.js */ "./wasm-bindings/pkg/index.js");


// exec wasm module
wasmExports["__webpack_init__"]()

/***/ })

});
//# sourceMappingURL=0.bundle.worker.js.map