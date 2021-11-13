#version 450

layout(location = 0) out vec4 col;

layout(set = 0, binding = 1) uniform Param {
	vec4 color;
};


void main() {
	col = color;
}
