#![allow(dead_code)]
#![allow(warnings, unused)]
mod rulox;

use rulox::Rulox;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: String) -> Rulox {
    let rulox = Rulox::new(source);

    rulox
}
