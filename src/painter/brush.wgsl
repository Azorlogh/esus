struct VertParams {
	transform: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> r_vparams: VertParams;

[[stage(vertex)]]
fn vert_main([[location(0)]] pos: vec2<f32>) -> [[builtin(position)]] vec4<f32> {
	return vec4<f32>(
		(r_vparams.transform * vec4<f32>(pos, 0.0, 1.0)).xy,
		1.0,
		1.0,
	);
}

struct FragParams {
	color: vec4<f32>;
};

[[group(0), binding(1)]]
var<uniform> r_fparams: FragParams;

[[stage(fragment)]]
fn frag_main() -> [[location(0)]] vec4<f32> {
	return r_fparams.color;
}
