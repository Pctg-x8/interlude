#version 450

layout(location = 0) in vec4 pos;
out gl_PerVertex { out vec4 gl_Position; };
layout(set = 0, binding = 0) uniform Matrixes { mat4 proj; };

void main()
{
	gl_Position = proj * pos;
}