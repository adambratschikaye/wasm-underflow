const fs = require('node:fs');
const wasmBuffer = fs.readFileSync('./target/wasm32-unknown-unknown/debug/wasm_underflow.wasm');
WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
  const { foo } = wasmModule.instance.exports;
  foo();
});
