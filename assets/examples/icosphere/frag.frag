#version 450

layout(location = 0) out vec4 target;
layout(constant_id = 0) const float r = 0.0f;
layout(constant_id = 1) const float g = 0.0f;
layout(constant_id = 2) const float b = 0.0f;
layout(constant_id = 3) const float a = 0.0f;

void main() { target = vec4(r, g, b, a); }
