#version 450

#ifdef GL_ES
precision mediump float;
#endif

layout(location = 2) in vec2 v_Uv;

layout(set = 2, binding = 0) uniform MaterialUniform {
    vec2 resolution; 
    float time;
    //float speed;
};

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(abs(sin(time)),0.0,0.0,1.0);
}
