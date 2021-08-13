#![allow(dead_code)]

use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::{WebGlRenderingContext};
use js_sys::WebAssembly;

#[path = "./res/shader_data.rs"] pub mod program_data;
#[path = "mod.rs"] pub mod gl_;
use gl_::*;
//pub mod program_data;
mod math;
use wasm_bindgen::prelude::*;

// TODO:
//  - get Shader file data
//  - bind Shader File to buffer

#[allow(dead_code)]
pub struct GlRender {
    program: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,

}
#[allow(dead_code)]
impl GlRender {
    pub fn new(gl_ctx: &WebGlRenderingContext) -> Self {
    
        //Shader::compile(new_shader_vs, gl_ctx, WebGlRenderingContext::VERTEX_SHADER);
        
        let program = gl_program::link_program(
            gl_ctx,
            &program_data::shader_vs,
            &program_data::shader_fs
        ).unwrap();
        
        let vert: [f32; 12] = [
            0., 1.,
            0., 0.,
            1., 1.,
            1., 1.,
            0., 0.,
            1., 0.,];
            
            let m_buf = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
            
            let vert_loc = vert.as_ptr() as u32 / 4;
            
            let v_arr = js_sys::Float32Array::new(&m_buf).subarray(vert_loc, vert_loc + vert.len() as u32);
            
            let b_rect = gl_ctx.create_buffer().ok_or("failed to create buffer").unwrap();
            
            gl_ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&b_rect));
            gl_ctx.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &v_arr, WebGlRenderingContext::STATIC_DRAW);
            
            Self { 
                u_color: gl_ctx.get_uniform_location(&program, "uColor").unwrap(),
                u_opacity: gl_ctx.get_uniform_location(&program, "uOpacity").unwrap(),
                u_transform: gl_ctx.get_uniform_location(&program, "uTransform").unwrap(),
                rect_vertice_ary_length: vert.len(),
                rect_vertice_buffer: b_rect,
                program: program,
            }
        }
        
        pub fn render(
            &self,
            gl_ctx: &WebGlRenderingContext,
            bottom: f32,
            top: f32,
            left: f32,
            right: f32,
            canvas_height: f32,
            canvas_width: f32,    
    ) {
    
        gl_ctx.use_program(Some(&self.program));

        gl_ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));

        // pub fn vertex_attrib_pointer_with_i32(
        //     this: &WebGlRenderingContext,
        //     indx: u32,
        //     size: i32,
        //     type_: u32,
        //     normalized: bool,
        //     stride: i32,
        //     offset: i32,
        // )

        gl_ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl_ctx.enable_vertex_attrib_array(0);
        
        gl_ctx.uniform4f(Some(&self.u_color),
            0.5,
            0.2,
            0.5,
            1.0,
        );
        gl_ctx.uniform1f(Some(&self.u_opacity), 1.);

        let trans_mat = math::translate_mat(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.
        );
        
        let scale_mat = math::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );

        let transform_mat = math::mult_matrix_4(scale_mat, trans_mat);
        gl_ctx.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        gl_ctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (self.rect_vertice_ary_length / 2) as i32);
        // gl_ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }
}