#![allow(dead_code)]
#![allow(warnings, unused)]
pub mod rulox;

use rulox::WebRulox;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: String) -> WebRulox {
    let rulox = WebRulox::new(source);

    rulox
}
