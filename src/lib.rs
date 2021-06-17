extern crate wasm_bindgen;
mod shader;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

mod setup;

#[wasm_bindgen(start)]
pub fn start()-> Result<(), JsValue> {

    // Setup of Canvas as a WebGL context from setup  
    let mut gl = setup::init_glctx().unwrap();

    // Creating a program linking the vertex shader and fragment to it
    
    let vertices: [f32; 9] = [-0.6, -0.7, 0.0, 
                            0.7, -0.7, 0.0, 
                            0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("Issues Initizialing a Buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertarray = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertarray,
            WebGlRenderingContext::STATIC_DRAW
        );
    }
    
    context.vertex_attrib_pointer_with_i32(0,3,WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.5, 0.5, 0.5, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32,);
    Ok(())
}



// Link Program
pub fn link_program(
    
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,) -> Result<WebGlProgram, String> {
    
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) 
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}