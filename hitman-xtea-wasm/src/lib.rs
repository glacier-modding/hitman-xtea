//! HITMAN 6/7 (2016/2018) packagedefinition handler.
//!
//! This crate was originally part of a larger crate;
//! It has been refactored slightly in 2019, and split
//! into its own crate to make it more accessible.
//!
//! Keys are not shipped in this, nor the delta.
//!
//! I promise when I wrote this I didn't intend to throw
//! random unsafe code into it, but you know, yolo wasm?

use wasm_bindgen::prelude::*;

#[cfg(feature="debugging")]
use std::panic;

#[cfg(feature="debugging")]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature="debugging")]
#[wasm_bindgen]
pub fn setup() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

/// Enciphers the buffer.
#[wasm_bindgen]
pub fn encipher(data: Box<[u8]>, delta: u32, header: Box<[u8]>, rounds: u32, key: Box<[u32]>) -> Option<Box<[u8]>> {
    let data_pointer = Box::into_raw(data);
    let unwrapped_data = unsafe { data_pointer.as_ref().expect("data is null") };
    if unwrapped_data.len() < 2 {
        panic!("data is < 2 for some reason");
    }

    let header_pointer = Box::into_raw(header);
    let unwrapped_header = unsafe { header_pointer.as_ref().expect("header is null") };
    let key_pointer = Box::into_raw(key);
    
    let unwrapped_key = unsafe { key_pointer.as_ref().expect("key is null") };
    if unwrapped_key.len() < 4 {
        panic!("key is < 2 for some reason");
    }

    let res = hitman_xtea::encipher_file(
        unwrapped_data,
        delta,
        &unwrapped_header,
        rounds,
        &unwrapped_key,
    );
    if res.is_err() {
        None
    } else {
        Some(res.unwrap().into_boxed_slice())
    }
}

/// Decipherd the buffer.
#[wasm_bindgen]
pub fn decipher(data: Box<[u8]>, delta: u32, header: Box<[u8]>, rounds: u32, key: Box<[u32]>) -> Option<Box<[u8]>> {
    let data_pointer = Box::into_raw(data);
    let unwrapped_data = unsafe { data_pointer.as_ref().expect("data is null") };
    if unwrapped_data.len() < 2 {
        panic!("data is < 2 for some reason");
    }

    let header_pointer = Box::into_raw(header);
    let unwrapped_header = unsafe { header_pointer.as_ref().expect("header is null") };
    let key_pointer = Box::into_raw(key);
    
    let unwrapped_key = unsafe { key_pointer.as_ref().expect("key is null") };
    if unwrapped_key.len() < 4 {
        panic!("key is < 2 for some reason");
    }

    let res = hitman_xtea::decipher_file(
        unwrapped_data,
        delta,
        &unwrapped_header,
        rounds,
        &unwrapped_key,
    );
    if res.is_err() {
        None
    } else {
        Some(res.unwrap().into_boxed_slice())
    }
}