extern crate wasm_bindgen;

// use js_sys::WebAssembly;
use wasm_bindgen::JsValue;

use web_sys::{WebGl2RenderingContext as GL, WebGlVertexArrayObject, WebGlBuffer};
use web_sys::{WebGlProgram, WebGlShader};
pub struct ProgramInfo {
    program_obj: WebGlProgram,
    vertex_color: u32,
    vertex_position: u32,
}

pub struct GlRender {
    programinfo: ProgramInfo,
    vert_count: i32,
    vertex_array_object: WebGlVertexArrayObject,

    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
}

impl GlRender {
    pub fn new(ctx: &GL) -> Result<GlRender, JsValue> {
        // let src_shader_vs = include_str!("glsl/vs.glsl");
        // let src_shader_fs = include_str!("glsl/fs.glsl");

        pub const src_shader_vs: &str = r##"#version 300 es
 
        in vec4 aVertexPosition;
        in vec4 aVertexColor;
        
        // uniform mat4 uModelViewMatrix;
        // uniform mat4 uProjectionMatrix;
        
        out vec4 vColor;
        
        void main() {
            gl_Position = aVertexPosition;
            vColor = aVertexColor;
        }
        "##;

        pub const src_shader_fs: &str = r##"#version 300 es 

        in lowp vec4 vColor;
        
        out lowp vec4 fragColor;
        
        void main() {
            fragColor = vColor;
        }
        "##;

        let compiled_shader_vs = GlRender::compile_shader(src_shader_vs, ctx, GL::VERTEX_SHADER)?;
        let compiled_shader_fs = GlRender::compile_shader(src_shader_fs, ctx, GL::FRAGMENT_SHADER)?;
        let program = GlRender::link_program(ctx, &compiled_shader_vs, &compiled_shader_fs)?;

        ctx.detach_shader(&program, &compiled_shader_vs);
        ctx.delete_shader(Some(&compiled_shader_vs));
        ctx.detach_shader(&program, &compiled_shader_fs);
        ctx.delete_shader(Some(&compiled_shader_fs));

    //
    // Position Buffers
    //

        let programinfo = ProgramInfo {
            vertex_color: ctx.get_attrib_location(&program, "aVertexColor") as u32,
            vertex_position: ctx.get_attrib_location(&program, "aVertexPosition") as u32,
            program_obj: program,
        };
        
        let vert: [f32; 9] = [
            0.7,  0.7, 0., 
            -0.7,  0.7, 0., 
            //  0.7, -0.7, 0., 
            -0.7, -0.7, 0.];
            
        let position_buffer = ctx.create_buffer().ok_or("failed to create vertex buffer")?;
        
        unsafe {
            let positions_array_buf_view =   js_sys::Float32Array::view(&vert);
            ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
            ctx.buffer_data_with_array_buffer_view( GL::ARRAY_BUFFER, &positions_array_buf_view, GL::STATIC_DRAW );
        }
    //
    // Color Buffers
    //
    
        let col: [f32; 16] = [
            1., 1., 1., 1.,
            1., 0., 1., 1.,
            1., 1., 0., 1.,
            0., 1., 0., 1.,
            ];
        
        let color_buffer = ctx
            .create_buffer()
            .ok_or("failed to create color buffer")?;
        
        unsafe{
            let colors = js_sys::Float32Array::view(&col);
            ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
            ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors, GL::STATIC_DRAW);
        }
            // ctx.vertex_attrib_pointer_with_i32(programinfo.vertex_color, 4, GL::FLOAT, false, 0, 0);
            // ctx.enable_vertex_attrib_array(programinfo.vertex_color);
//          Above is init Code
// ----------------------------------------------------------------------------------------------------
//          Rendering Code

        let vert_count = (vert.len() / 3) as i32;

        // ctx.vertex_attrib_pointer_with_i32(programinfo.vertexColor, 4, GL::FLOAT, false, 0, 0);
        let vertex_array_object = ctx
            .create_vertex_array()
            .ok_or("Failed to create VAO")
            .unwrap();
        ctx.bind_vertex_array(Some(&vertex_array_object));


        Ok(GlRender {
            programinfo: programinfo,
            vert_count:vert_count as i32,
            vertex_array_object:vertex_array_object,

            position_buffer: position_buffer,
            color_buffer: color_buffer,
        })
    }

    pub fn draw(&self, ctx: &GL) {
        ctx.clear_color(0.2, 0.3, 0.5, 1.0);
        ctx.clear_depth(1.0);
        ctx.enable(GL::DEPTH_TEST);
        ctx.clear(GL::COLOR_BUFFER_BIT);

        ctx.bind_vertex_array(Some(&self.vertex_array_object));
        // Position buffer
        ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        ctx.vertex_attrib_pointer_with_i32(self.programinfo.vertex_position, 3, GL::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(self.programinfo.vertex_position);

        ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        ctx.vertex_attrib_pointer_with_i32(self.programinfo.vertex_color, 4, GL::FLOAT, false, 0, 0);
        ctx.enable_vertex_attrib_array(self.programinfo.vertex_color);

        ctx.bind_vertex_array(Some(&self.vertex_array_object));

        ctx.use_program(Some(&self.programinfo.program_obj));

        ctx.draw_arrays(GL::TRIANGLES, 0, self.vert_count);
    }

    fn compile_shader(source: &str, context: &GL, shader_type: u32) -> Result<WebGlShader, String> {
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
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
    fn link_program(
        context: &GL,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
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
