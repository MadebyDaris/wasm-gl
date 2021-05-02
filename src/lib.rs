extern crate wasm_bindgen;

use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen(start)]
pub fn start()->Result<(), JsValue> {

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canv`s").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Setup of Canvas as a WebGL context to put our things in
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    
    let vertShader = compile_shader(
        &context, 
        WebGlRenderingContext::VERTEX_SHADER,
        
        r#"
        attribute vec4 pos;"
        void main() {
            gl_position = pos;
        }
        "#,
    );
}

pub fn compile_shader(
    context: WebGlRenderingContext,
    shader_type: u32,
    shader_source_glsl: &str){

    }