#version 450

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec4 inColor;

layout(location = 0) out vec4 fragColor;

layout(binding = 0) uniform UniformTransforms {
    mat4 projection;
} transform;

void main() {
    gl_Position = transform.projection * vec4(inPosition, 0.0, 1.0);

    fragColor = inColor;
}
