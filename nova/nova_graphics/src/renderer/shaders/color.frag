#version 450

layout(location = 0) in vec4 inColor;

layout(location = 0) out vec4 outColor;

layout(push_constant) uniform PushConstants {
  vec4 tint;
} pushed;

void main() {
  outColor = inColor;
}
