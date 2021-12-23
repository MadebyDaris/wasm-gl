extern crate wasm_bindgen;

use std::str;
use std::io::prelude::*;    

// object-oriented programing test
use web_sys::{WebGlRenderingContext, WebGlShader};

pub fn compile_shader( 
    source: &str,
    context: &WebGlRenderingContext,
    shader_type: u32
) -> Result<WebGlShader, String> {

    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {

        Ok(shader)}

    else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
    }
}