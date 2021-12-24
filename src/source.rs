pub const SHADER_VS: &str = r##"#version 300 es
 
in vec4 position;

void main() {

    gl_Position = position;
}
"##;

pub const SHADER_FS: &str = r##"#version 300 es
    
precision highp float;
out vec4 outColor;

void main() {
    outColor = vec4(1, 1, 1, 1);
}
"##;

// pub const shader_fs: &str =  r#"void main() {
//     gl_FragColor = vec4(1.0, 0.6, 1.0, 1.0);
// }
// "#;