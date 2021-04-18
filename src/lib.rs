extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let rc = format!("Hello, {}!", name);
    unsafe {alert(&rc)};
}

// use web_sys::*;
// use web_sys::WebGlRenderingContext as GL;
