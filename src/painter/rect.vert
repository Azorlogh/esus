#version 450

layout(location = 0) in vec2 pos;

layout(set = 0, binding = 0) uniform Param {
	vec4 bounds;
};

void main() {
	gl_Position = vec4(pos*(bounds.zw-bounds.xy)+bounds.xy, 0.0, 1.0);
	// gl_Position = vec4(pos, 0.0, 1.0);
}
