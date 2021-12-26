extern crate wasm_bindgen;
use wasm_bindgen::JsCast;
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
            Some(x) =>{ let ctx = x.dyn_into::<GL>()?;
                Ok( GlClient{ render:renderGl::GlRender::new(&ctx)?,
                    ctx: ctx })},
            None => {
                return Err(JsValue::from("Error couldn't get the canvas element."))
            }
        }
    }
    
    pub fn render_client( &self ) {

        self.render.draw(&self.ctx)
        
    }
}