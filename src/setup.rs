extern crate wasm_bindgen;

#[path = "lib/mod.rs"] mod shader;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use web_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub struct SetupCtx {
    f: String,
    v: String
}

impl SetupCtx {
    pub fn new( f: String, v: String ) -> SetupCtx {
        let (f,v ) = ("/res/shader.fs", "/res/shader.vs");
        // f: f.to_string();
        SetupCtx { f: f.to_string(), v: f.to_string() } 
    }

    pub fn init_gl_ctx(&mut self) -> Result<WebGlRenderingContext, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("page").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        // Setup of Canvas as a WebGL context to put our things in
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;  

        // let mut vert = shader::gl_shader::Shader::new(&self.v);
        // let mut vert_shader = vert.compile(&context, WebGlRenderingContext::VERTEX_SHADER)?;

        // // we have 2 variables as frg is of type Shader and not WebGlShader, and so link_program doesnt accept it
        // let mut frg = shader::gl_shader::Shader::new(&self.f);
        // let mut frg_shader = frg.compile(&context, WebGlRenderingContext::FRAGMENT_SHADER)?;

        // let program = shader::gl_program::GlProgram::link_program(&context, &vert_shader, &frg_shader)
        // .unwrap();

        // context.use_program(Some(&program));

        context.enable(WebGlRenderingContext::BLEND);
        context.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
        context.clear_color(0.5, 0.5, 0.5, 1.0);
        context.clear_depth(1.);

        Ok(context)

    }
}