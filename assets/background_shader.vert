#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform View {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Material{
    vec2 resolution; 
    float time;
    //float speed;
};

void main() {
    gl_Position = ViewProj * vec4(Vertex_Position, 1.0);
}
