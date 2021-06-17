extern crate wasm_bindgen;
use super::shader;
use super::program;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn init_glctx()-> Result<(), JsValue> {

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Setup of Canvas as a WebGL context to put our things in
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let mut vert = shader::Shader::new(&v);
    let vert_shader = vert.compile(&context, WebGlRenderingContext::VERTEX_SHADER)?;

    // we have 2 variables as frg is of type Shader and not WebGlShader, and so link_program doesnt accept it
    let mut frg = shader::Shader::new(&f);
    let frg_shader = frg.compile(&context, WebGlRenderingContext::FRAGMENT_SHADER)?;
    
    let (f,v ) = ("/res/shader.fs", "/res/shader.vs");

    let program = shader::link_program(&context, &vert_shader, &frg_shader)?;

    context.use_program(Some(&program));
}