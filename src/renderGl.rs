extern crate wasm_bindgen;

// use js_sys::WebAssembly;
use wasm_bindgen::JsValue;
// use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
    #[path = "./source.rs"] mod source;

use web_sys::{WebGlProgram, WebGlShader};
    
use web_sys::WebGl2RenderingContext as GL;

pub struct GlRender {
    vert_count:  i32
}

impl GlRender {
    pub fn new(ctx: &GL) ->Result<GlRender, JsValue> {
        let compiled_shader_vs = GlRender::compile_shader(source::SHADER_VS, ctx, GL::VERTEX_SHADER)?;
        let compiled_shader_fs = GlRender::compile_shader(source::SHADER_FS, ctx, GL::FRAGMENT_SHADER)?;
        let program = GlRender::link_program(ctx, &compiled_shader_vs, &compiled_shader_fs)?;

        ctx.use_program(Some(&program));

        let vert: [f32; 9] = [
        -0.7, -0.7, 0.0, 
        0.7, -0.7, 0.0, 
        0.0, 0.7, 0.0];

        let position_attribute_location = ctx.get_attrib_location(&program, "position");
        let buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
        ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view  = js_sys::Float32Array::view(&vert);
            ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &positions_array_buf_view , GL::STATIC_DRAW);
        }

        // VAO
        let vertex_array_object = ctx.create_vertex_array().ok_or("Failed to create VAO").unwrap();
        ctx.bind_vertex_array(Some(&vertex_array_object));

        ctx.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(position_attribute_location as u32);

        ctx.bind_vertex_array(Some(&vertex_array_object));

        let vert_count = (vert.len() / 3) as i32;
        Ok( GlRender { vert_count })
    }
    
    pub fn draw(&self, ctx: &GL) {
        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear(GL::COLOR_BUFFER_BIT);
        ctx.draw_arrays(GL::TRIANGLES, 0, self.vert_count);
    }

    fn compile_shader( 
        source: &str,
        context: &GL,
        shader_type: u32) -> Result<WebGlShader, String> {
    
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
    
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
    
        if context
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
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
    
    fn link_program(
        context: &GL,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,) -> Result<WebGlProgram, String> {
    
        let new_program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;
    
        context.attach_shader(&new_program, vert_shader);
        context.attach_shader(&new_program, frag_shader);
        context.link_program(&new_program);
    
        if context
            .get_program_parameter(&new_program, GL::LINK_STATUS)
            .as_bool()  
            .unwrap_or(false) 
        {
            Ok(new_program)
        } else {
            Err(context
                .get_program_info_log(&new_program)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}