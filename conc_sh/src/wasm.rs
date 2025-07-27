use wasm_bindgen::prelude::*;

use crate::translate;

#[wasm_bindgen]
pub fn translate_word(word: &str) -> String {
    translate(word)
}