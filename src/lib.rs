extern crate wasm_bindgen;
extern crate console_error_panic_hook;
#[path = "./setup.rs"] mod setup;
#[path = "./lib/gl_render.rs"] mod program;
#[path = "./lib/res/shader_data.rs"] mod data;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct MyClient {
    program2D: program::GlRender,
    ctx: WebGlRenderingContext
}

#[wasm_bindgen]
impl MyClient {
    #[wasm_bindgen(constructor)]
    pub fn new () -> Self {
        console_error_panic_hook::set_once();
        let mut gl = setup::SetupCtx::new(String::from(data::shader_vs), String::from(data::shader_fs));
        let ctx = setup::SetupCtx::init_gl_ctx(&mut gl).unwrap();
        Self {
            ctx: ctx,
            program2D: program::GlRender::new(&gl.init_gl_ctx().unwrap())
        }
    }

    pub fn update( &self)  {
    }

    pub fn render( &self ) {
        self.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        self.program2D.render(
            &self.ctx,
            0.,
            10.,
            0.,
            10.,
            100.,
            100.,
        )
    }
}

// pub fn start()-> Result<(), JsValue> {

//     // Setup of Canvas as a WebGL context from setup  
    
//     // Creating a program linking the vertex shader and fragment to it
    
//     let vertices: [f32; 9] = [-0.6, -0.7, 0.0, 
//                             0.7, -0.7, 0.0, 
//                             0.0, 0.7, 0.0];

//     let buffer = ctx.create_buffer().ok_or("Issues Initizialing a Buffer")?;
//     ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

//     unsafe {
//         let vertarray = js_sys::Float32Array::view(&vertices);

//         ctx.buffer_data_with_array_buffer_view(
//             WebGlRenderingContext::ARRAY_BUFFER,
//             &vertarray,
//             WebGlRenderingContext::STATIC_DRAW
//         );
//     }
    
//     ctx.vertex_attrib_pointer_with_i32(0,3,WebGlRenderingContext::FLOAT, false, 0, 0);
//     ctx.enable_vertex_attrib_array(0);
//     ctx.draw_arrays(
//         WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32,);
//     Ok(())
// }