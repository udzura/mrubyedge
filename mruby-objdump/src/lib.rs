#![feature(ascii_char)]
#![feature(ascii_char_variants)]

pub mod error;
pub mod format;
pub mod insn;
pub mod marker;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn debug_output() {
    let bin = include_bytes!("../examples/hi.mrb");
    match crate::format::load(bin) {
        Ok(_) => alert("parsed!"),
        Err(err) => alert(&format!("{:?}", err)),
    }
}
