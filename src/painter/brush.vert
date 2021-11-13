#version 450

layout(location = 0) in vec2 pos;

layout(set = 0, binding = 0) uniform Param {
	mat4 transform;
};

void main() {
	gl_Position = vec4(
		(transform * vec4(pos, 0.0, 1.0)).xy,
		1.0,
		1.0
	);
}
