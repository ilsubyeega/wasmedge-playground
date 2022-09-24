cargo build --target wasm32-wasi
# FIXME: For windows, see https://github.com/WasmEdge/WasmEdge/issues/953
wasmedgec target/wasm32-wasi/debug/rs_simple_function.wasm target/aot.wasm