extern crate wasm_bindgen;

#[path = "../shader/gl_shader.rs"] mod gl_shader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use web_sys::*; 

#[wasm_bindgen(start)]
pub struct GlProgram {
    context: web_sys::WebGlRenderingContext,
    vert_shader: WebGlShader,
    frag_shader: WebGlShader
}

impl GlProgram {
    // Constructor
    pub fn new(
        context: WebGlRenderingContext, 
        vert_shader: WebGlShader, 
        frag_shader: WebGlShader
    ) -> GlProgram {
        GlProgram { context, vert_shader, frag_shader }
    }
    
    pub fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,) -> Result<WebGlProgram, String> {
        
        let GlProgram = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&GlProgram, vert_shader);
        context.attach_shader(&GlProgram, frag_shader);
        context.link_program(&GlProgram);

        if context
            .get_program_parameter(&GlProgram, WebGlRenderingContext::LINK_STATUS)
            .as_bool()  
            .unwrap_or(false) 
        {
            Ok(GlProgram)
        } else {
            Err(context
                .get_program_info_log(&GlProgram)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}