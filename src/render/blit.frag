#version 450

layout(location = 0) out vec4 col;

layout(set = 0, binding = 0) uniform texture2D t_screen;
layout(set = 0, binding = 1) uniform sampler s_screen;

void main() {
	col = texture(sampler2D(t_screen, s_screen), uv);
}
