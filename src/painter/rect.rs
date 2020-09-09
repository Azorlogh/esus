use super::RenderCtx;
use crate::util;
use zerocopy::AsBytes as _;

const UNIFORM_SIZE: wgpu::BufferAddress = 4 * std::mem::size_of::<f32>() as wgpu::BufferAddress;
const VERTEX_SIZE: wgpu::BufferAddress = 2 * std::mem::size_of::<f32>() as wgpu::BufferAddress;

#[allow(dead_code)]
pub struct Rect {
	vs_module: wgpu::ShaderModule,
	fs_module: wgpu::ShaderModule,
	u_buf: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
	pipeline: wgpu::RenderPipeline,
	vertex_buf: wgpu::Buffer,
}

impl Rect {
	pub fn new(device: &wgpu::Device) -> Rect {
		let vs_module = device.create_shader_module(&util::compile_shader(
			include_str!("rect.vert"),
			shaderc::ShaderKind::Vertex,
		));
		let fs_module = device.create_shader_module(&util::compile_shader(
			include_str!("rect.frag"),
			shaderc::ShaderKind::Fragment,
		));

		let u_buf = device.create_buffer_with_data(
			[10.0f32, 10.0, 100.0, 20.0].as_bytes(),
			wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
		);

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			bindings: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStage::FRAGMENT,
				ty: wgpu::BindingType::UniformBuffer { dynamic: false },
			}],
			label: None,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			bindings: &[wgpu::Binding {
				binding: 0,
				resource: wgpu::BindingResource::Buffer {
					buffer: &u_buf,
					range: 0..UNIFORM_SIZE,
				},
			}],
			label: None,
		});

		let vertex_buf = device.create_buffer_with_data(
			[-1.0f32, -1.0, 3.0, -1.0, -1.0, 3.0].as_bytes(),
			wgpu::BufferUsage::VERTEX,
		);

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			bind_group_layouts: &[&bind_group_layout],
		});

		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			layout: &pipeline_layout,
			vertex_stage: wgpu::ProgrammableStageDescriptor {
				module: &vs_module,
				entry_point: "main",
			},
			fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
				module: &fs_module,
				entry_point: "main",
			}),
			rasterization_state: Some(wgpu::RasterizationStateDescriptor {
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: wgpu::CullMode::None,
				depth_bias: 0,
				depth_bias_slope_scale: 0.0,
				depth_bias_clamp: 0.0,
			}),
			primitive_topology: wgpu::PrimitiveTopology::TriangleList,
			color_states: &[wgpu::ColorStateDescriptor {
				format: wgpu::TextureFormat::Bgra8UnormSrgb,
				color_blend: wgpu::BlendDescriptor {
					src_factor: wgpu::BlendFactor::SrcAlpha,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
					operation: wgpu::BlendOperation::Add,
				},
				alpha_blend: wgpu::BlendDescriptor {
					src_factor: wgpu::BlendFactor::One,
					dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
					operation: wgpu::BlendOperation::Add,
				},
				write_mask: wgpu::ColorWrite::ALL,
			}],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: wgpu::IndexFormat::Uint16,
				vertex_buffers: &[wgpu::VertexBufferDescriptor {
					stride: VERTEX_SIZE,
					step_mode: wgpu::InputStepMode::Vertex,
					attributes: &[wgpu::VertexAttributeDescriptor {
						format: wgpu::VertexFormat::Float2,
						offset: 0,
						shader_location: 0,
					}],
				}],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		Rect {
			vs_module,
			fs_module,
			u_buf,
			bind_group,
			pipeline,
			vertex_buf,
		}
	}

	pub fn fill(&self, render_ctx: &mut RenderCtx, b0: (f32, f32), b1: (f32, f32)) {
		// set uniforms
		{
			let buf = render_ctx.device.create_buffer_with_data(
				[b0.0, b0.1, b1.0, b1.1].as_bytes(),
				wgpu::BufferUsage::COPY_SRC,
			);

			render_ctx
				.encoder
				.copy_buffer_to_buffer(&buf, 0, &self.u_buf, 0, UNIFORM_SIZE);
		}

		let mut rpass = render_ctx
			.encoder
			.begin_render_pass(&wgpu::RenderPassDescriptor {
				color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &render_ctx.frame.view,
					resolve_target: None,
					load_op: wgpu::LoadOp::Load,
					store_op: wgpu::StoreOp::Store,
					clear_color: wgpu::Color {
						r: 0.1,
						g: 1.0,
						b: 0.3,
						a: 1.0,
					},
				}],
				depth_stencil_attachment: None,
			});

		rpass.set_pipeline(&self.pipeline);
		rpass.set_bind_group(0, &self.bind_group, &[]);
		rpass.set_vertex_buffer(0, &self.vertex_buf, 0, 0);
		rpass.draw(0..3, 0..1);
	}
}
