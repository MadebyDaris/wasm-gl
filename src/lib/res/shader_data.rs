pub const shader_vs: &str = r#"
    attribute vec4 aPosition;
    uniform mat4 uTransform;

    void main() {
        gl_Position = uTransform * aPosition;
    }
"#;

pub const shader_fs: &str = r#"
    precision mediump float;
    
    uniform vec4 uColor;
    uniform float uOpacity;

    void main() {
        gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
    }
"#;

// pub const shader_fs: &str =  r#"void main() {
//     gl_FragColor = vec4(1.0, 0.6, 1.0, 1.0);
// }
// "#;