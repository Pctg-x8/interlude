#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout(location = 0) in vec4 uv;
layout(location = 0) out vec4 target;

layout(set = 0, binding = 1) uniform sampler2D tex;

void main() { target = vec4(texture(tex, uv.xy).r); }
