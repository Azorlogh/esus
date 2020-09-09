#version 450

layout(location = 0) out vec4 col;

layout(set = 0, binding = 0) uniform Param {
	vec4 bounds;
};

void main() {
	vec2 coord = gl_FragCoord.xy;
	vec4 color = vec4(0);
	if (coord == clamp(coord, bounds.xy, bounds.zw)) {
		color = vec4(1,0,0,1);
	}
	col = color;
}
