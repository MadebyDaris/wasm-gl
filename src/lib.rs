use std::task::Context;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
#[path = "./render.rs"] mod render;
#[path = "./gl/glshader.rs"] mod shader;
#[path = "./gl/glprogram.rs"] mod program;


extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlShader, window, Document, HtmlElement,
Element,console,Node,HtmlCanvasElement,HtmlCollection,WebGlBuffer,WebGlUniformLocation,CanvasRenderingContext2d};
use web_sys::WebGlRenderingContext as GL;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_ctx(c: HtmlElement) -> Result<GL, JsValue> {
    // let window = window().unwrap();
    // let document = window.document().unwrap();
    // let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas:HtmlCanvasElement = c.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl")?
    .unwrap().dyn_into::<GL>()?;

    context.enable(GL::BLEND);
    context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    context.clear_color(0.5, 0.2, 0.5, 1.0);
    context.clear_depth(1.);
    Ok(context)
}

#[wasm_bindgen]
pub struct GlClient {
    program2D: render::gl_render,
    context: GL
}
#[wasm_bindgen]
impl GlClient {
    #[wasm_bindgen(constructor)]
    pub fn new(c: HtmlElement) -> Self {
        console_error_panic_hook::set_once();

        let context: GL = init_ctx(c).unwrap();
        Self{ program2D: render::gl_render::new(&context), context: context}
    }
    pub fn update( &self ) {
        // Ok(());
    }
    pub fn render( &self ) {
        self.context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.program2D.render(
            &self.context,
            0.,
            10.,
            0.,
            10.,
            20.,
            30.,
        )
    }
    
}

