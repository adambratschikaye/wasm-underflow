const fs = require('node:fs');
const wasmBuffer = fs.readFileSync('./target/wasm32-unknown-unknown/debug/wasm_underflow.wasm');
WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
  const { main } = wasmModule.instance.exports;
  main();
});
