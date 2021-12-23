extern crate wasm_bindgen;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
    #[path = "./gl/glshader.rs"] mod shader;
    #[path = "./gl/glprogram.rs"] mod program;
    #[path = "./gl/source.rs"] mod source; 
    mod math;

use web_sys::{WebGlProgram, WebGlRenderingContext ,WebGlBuffer,WebGlUniformLocation};
use js_sys::WebAssembly;
use WebGlRenderingContext as GL;
#[wasm_bindgen]
pub struct gl_render {
    programObj: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}
#[wasm_bindgen]
impl gl_render {
    pub fn new(ctx: &GL) -> Self {
        let programObj = program::link_program(ctx, source::shader_vs, source::shader_fs).unwrap();
        
        let vert: [f32; 12] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 1., // x, y
            0., 0., // x, y
            1., 0., // x, y
        ];
    
    let indices = [0, 1, 2];

    let mut m_buff = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    
    let vert_location = vert.as_ptr() as u32 / 4;

    let v_arr =  js_sys::Float32Array::new(&m_buff).subarray(vert_location, vert_location + vert.len() as u32);

    let buffer_rect = ctx.create_buffer().ok_or("failed to create buffer").unwrap();

    ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_rect));
    ctx.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &v_arr, WebGlRenderingContext::STATIC_DRAW);
    Self {
        u_color: ctx.get_uniform_location(&programObj, "uColor").unwrap(),
        u_opacity: ctx.get_uniform_location(&programObj, "uOpacity").unwrap(),
        u_transform: ctx.get_uniform_location(&programObj, "uTransform").unwrap(),        
        rect_vertice_ary_length: vert.len(),
        rect_vertice_buffer: buffer_rect,
        programObj: programObj,
        }
    } 
    #[wasm_bindgen]
    pub fn render(&self,
            ctx: &GL,
            bottom: f32,
            top: f32,
            left: f32,
            right: f32, 
            canvas_height: f32,
            canvas_width: f32,) {
        ctx.use_program(Some(&self.programObj));
        ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        ctx.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(0);

        ctx.uniform1f(Some(&self.u_opacity), 1.);

        let trans_mat = math::translate_mat(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.
        );
        
        let scale_mat = math::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.
        );

        let transform_mat = math::mult_matrix_4(scale_mat, trans_mat);
        ctx.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        ctx.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_length / 2) as i32);

    }
}