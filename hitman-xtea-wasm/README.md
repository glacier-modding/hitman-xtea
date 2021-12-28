# Hitman Package Definition Viewer/Editor (WASM)

Simple viewer/editor for Hitman 2016/2016 `packagedefinitions.txt` (and `thumbs.dat`).  Leverages the [`hitman-xtea`](../..hitman-xtea) crate and provides a simple softare CRC32 implementation to allow your browser to do the work.

Still requires a heap of browser related work on the other end, which is way less pleasant.

## Building

Needs WASM kit for Rust.  I don't use the WASM packer, I use bindgen directly, per the batch or shell files.  For debugging and whatnot, set the `debugging` feature on this crate and it'll add panics (enjoy the console logs).  Nothing fancy, but it works.