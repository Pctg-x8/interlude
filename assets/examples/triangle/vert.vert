#version 450

layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 color_o;
out gl_PerVertex { out vec4 gl_Position; };

void main()
{
	gl_Position = pos;
	color_o = color;
}