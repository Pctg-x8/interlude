#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Vertex
layout(location = 0) in vec4 base_vert;		// Position and UV
// Instance
layout(location = 1) in vec4 pos_scaling;
layout(location = 2) in vec4 uv_scaling;	// zw: unused

layout(std140, set = 0, binding = 0) uniform Transforming
{
	mat4 matrix_pp;
};

layout(location = 0) out vec4 uv;
out gl_PerVertex { vec4 gl_Position; };

void main()
{
	gl_Position = fma(base_vert, vec4(pos_scaling.zw, 1.0f, 1.0f), vec4(pos_scaling.xy, 0.0f, 0.0f)) * matrix_pp;
	uv = fma(base_vert, uv_scaling.zwxy, uv_scaling.xyzw);
}
