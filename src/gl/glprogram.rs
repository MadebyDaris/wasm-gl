extern crate wasm_bindgen;
mod glshader;
mod source; 

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use web_sys::*;


pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &str,
    frag_shader: &str,) -> Result<WebGlProgram, String> {

    let sh_vs = glshader::compile_shader(source::shader_vs, context, WebGlRenderingContext::VERTEX_SHADER).unwrap();
    
    let sh_fs = glshader::compile_shader(source::shader_fs, context, WebGlRenderingContext::FRAGMENT_SHADER).unwrap();

    let InitShaderProgram = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&InitShaderProgram, &sh_vs);
    context.attach_shader(&InitShaderProgram, &sh_fs);
    context.link_program(&InitShaderProgram);

    if context
        .get_program_parameter(&InitShaderProgram, WebGlRenderingContext::LINK_STATUS)
        .as_bool()  
        .unwrap_or(false) 
    {
        Ok(InitShaderProgram)
    } else {
        Err(context
            .get_program_info_log(&InitShaderProgram)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}