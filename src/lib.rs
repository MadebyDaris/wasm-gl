extern crate wasm_bindgen;
use wasm_bindgen::JsCast;
use std::option;
use js_sys::Object;
mod renderGl;
// #[path = "./main.rs"] mod main;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement,HtmlCanvasElement, WebGl2RenderingContext as GL};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GlClient {
    render: renderGl::GlRender,
    ctx: GL,
}

#[wasm_bindgen]
impl GlClient {
    #[wasm_bindgen(constructor)]
    pub fn new(c: HtmlElement) -> Result<GlClient, JsValue> {
        console_error_panic_hook::set_once();
        
        let canvas:HtmlCanvasElement = c.dyn_into::<HtmlCanvasElement>()?;
        match canvas.get_context("webgl2")? {
                // The division was valid
                Some(x) =>{ let ctx = x.dyn_into::<GL>()?;
                    Ok( GlClient{ render:renderGl::GlRender::new(&ctx).unwrap(),
                        ctx: ctx })},
                // The division was invalid
                None => {
                    return Err(JsValue::from("Error counld't get the canvas elemet."))
                }
        }

    }
    
    pub fn renderClient( &self ) {

        self.render.draw(&self.ctx)
        
    }
}