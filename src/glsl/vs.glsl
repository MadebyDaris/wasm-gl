#version 300 es
 
in vec4 aVertexPosition;
in vec4 aVertexColor;

// uniform mat4 uModelViewMatrix;
// uniform mat4 uProjectionMatrix;

out vec4 vColor;

void main() {
    gl_Position = aVertexPosition;
    vColor = aVertexColor;
}