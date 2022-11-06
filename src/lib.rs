use wasm_bindgen::prelude::*;
use cjp::AsCJp;

#[wasm_bindgen]
pub fn cjp(jp: &str) -> String {
  jp.to_string().cjp()
}
