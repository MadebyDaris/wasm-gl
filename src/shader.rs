extern crate wasm_bindgen;

use std::{fs, str};
use std::io::prelude::*;

// object-oriented programing test
use web_sys::{WebGlRenderingContext, WebGlShader};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;



#[wasm_bindgen(start)]
pub struct Shader {
    code: String,
}

impl Shader {
    // Constructor
    pub fn new(path: &str) -> Shader {
        // assert!(vertexPath = String::as_str('{}'));
        // Shader { vertexPath: "hello", fragmentPath: "hello" };
        // Open files
      
        let code = fs::read_to_string(path).expect("Something went wrong reading the file");
        
        Shader { code }
    }

    // Compile Shader
    pub fn compile( 
        &mut self, 
        context: &WebGlRenderingContext,
        shader_type: u32
    ) -> Result<WebGlShader, String> {
    
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.shader_source(&shader, &self.code);
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
}