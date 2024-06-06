## Prerequisites

```
$ cargo --version
cargo 1.78.0 (54d8815d0 2024-03-26)

$ wasmtime --version
wasmtime-cli 21.0.1
```

## Build
```
$ cargo build --target wasm32-wasi
$ cargo build --target wasm32-unknown-unknown
```

## Run
```
$ wasmtime run --invoke foo target/wasm32-wasi/debug/wasm_underflow.wasm
```

Running should have an output like:
```
current highest ffffe6a0
current highest ffffffb0
checking vecs
thread '<unnamed>' panicked at src/lib.rs:63:13:
assertion `left == right` failed
  left: 171
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: failed to run main module `target/wasm32-wasi/debug/wasm_underflow.wasm`

Caused by:
    0: failed to invoke `foo`
    1: error while executing at wasm backtrace:
           0: 0xa84a - wasm_underflow.wasm!__rust_start_panic
           1: 0xa7a4 - wasm_underflow.wasm!rust_panic
           2: 0xa6dd - wasm_underflow.wasm!std::panicking::rust_panic_with_hook::hd3fb69bc0aea298a
           3: 0x996a - wasm_underflow.wasm!std::panicking::begin_panic_handler::{{closure}}::h4d99b90b43f79472
           4: 0x98cd - wasm_underflow.wasm!std::sys_common::backtrace::__rust_end_short_backtrace::h5691573a73161cb1
           5: 0xa0cd - wasm_underflow.wasm!rust_begin_unwind
           6: 0xee43 - wasm_underflow.wasm!core::panicking::panic_fmt::hdb62f5cdb45533e4
           7: 0x10247 - wasm_underflow.wasm!core::panicking::assert_failed_inner::hcf1985c073eb6fd3
           8: 0x4ccf - wasm_underflow.wasm!core::panicking::assert_failed::hdf150a194974dcdc
           9: 0x5a57 - wasm_underflow.wasm!foo
          10: 0x118c7 - wasm_underflow.wasm!foo.command_export
       note: using the `WASMTIME_BACKTRACE_DETAILS=1` environment variable may show more debugging information
    2: wasm trap: wasm `unreachable` instruction executed

```


## Explanation
In the main function:
```
    let vecs = setup();
    let _dummy = [0_i32; 368 * 500 + 364];

    let mut init = [[0_u8; SIZE]; SIZE];
    let count = 900;
    bar(count, &mut init);

    println!("checking vecs");
    for v in vecs {
        for b in v {
            assert_eq!(b, 0, "Vector has non-zero value 0x{:x}", b);
        }
    }
```
`setup` creates some vectors which are all initialized with `0`s.
`bar` doesn't touch those vectors, but it recurses which underflows the rust stack and corrupts the vectors at the end of the main Wasm memory. `dummy` preallocates the stack to reduce the recursion required. After `bar` completes, fail the assertion because one of the vectors has had its initial value changed even though it should be immutable.

## Affected Platforms

| Platform | Affected | Version        |
| -------- | -------- | -------------- |
| wasmtime | ✅        | 21.0.1         |
| wasmer   | ❌        | 4.3.1          |
| wasmedge | ✅        | 0.14.0         |
| Node     | ✅        | 21.7.3         |
| Chrome   | ✅        | 125.0.6422.142 |
