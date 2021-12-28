cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target no-modules ./target/wasm32-unknown-unknown/release/hitwasm_xtea.wasm --out-dir="built/hitwasm/"