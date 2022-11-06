use wasm_bindgen::prelude::*;
use cjp::AsCJp;

#[wasm_bindgen]
pub fn cjp(jp: &str) {
    let s = jp.to_string();
    return format!("{}", s.cjp());
}
