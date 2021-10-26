use std::borrow::Cow;

use super::RenderCtx;
use crate::{render::Renderer, util};
use wgpu::util::DeviceExt;
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
	pub fn new(renderer: &Renderer) -> Rect {
		let vs_module = renderer
			.device
			.create_shader_module(&wgpu::ShaderModuleDescriptor {
				label: None,
				source: wgpu::ShaderSource::SpirV(Cow::Owned(util::compile_shader(
					include_str!("rect.vert"),
					shaderc::ShaderKind::Vertex,
				))),
			});
		let fs_module = renderer
			.device
			.create_shader_module(&wgpu::ShaderModuleDescriptor {
				label: None,
				source: wgpu::ShaderSource::SpirV(Cow::Owned(util::compile_shader(
					include_str!("rect.frag"),
					shaderc::ShaderKind::Fragment,
				))),
			});

		let u_buf = renderer
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: [10.0f32, 10.0, 100.0, 20.0].as_bytes(),
				usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
			});

		let bind_group_layout =
			renderer
				.device
				.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
					entries: &[wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::VERTEX,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					}],
					label: None,
				});

		let bind_group = renderer
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &bind_group_layout,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: u_buf.as_entire_binding(),
				}],
				label: None,
			});

		let vertex_buf = renderer
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: [0.0f32, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0].as_bytes(),
				usage: wgpu::BufferUsages::VERTEX,
			});

		let pipeline_layout =
			renderer
				.device
				.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
					label: None,
					bind_group_layouts: &[&bind_group_layout],
					push_constant_ranges: &[],
				});

		let pipeline = renderer
			.device
			.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
				label: None,
				layout: Some(&pipeline_layout),
				vertex: wgpu::VertexState {
					module: &vs_module,
					entry_point: "main",
					buffers: &[wgpu::VertexBufferLayout {
						array_stride: VERTEX_SIZE,
						step_mode: wgpu::VertexStepMode::Vertex,
						attributes: &[wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float32x2,
							offset: 0,
							shader_location: 0,
						}],
					}],
				},
				fragment: Some(wgpu::FragmentState {
					module: &fs_module,
					entry_point: "main",
					targets: &[renderer.surface_cfg.format.into()],
				}),
				primitive: wgpu::PrimitiveState {
					cull_mode: None,
					topology: wgpu::PrimitiveTopology::TriangleStrip,
					..Default::default()
				},
				depth_stencil: None,
				multisample: wgpu::MultisampleState::default(),
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

	pub fn fill(&self, render_ctx: &mut RenderCtx, mut b0: (f32, f32), mut b1: (f32, f32)) {
		b0.0 = b0.0 / render_ctx.size.width as f32 * 2.0 - 1.0;
		b0.1 = 1.0 - b0.1 / render_ctx.size.width as f32 * 2.0;
		b1.0 = b1.0 / render_ctx.size.width as f32 * 2.0 - 1.0;
		b1.1 = 1.0 - b1.1 / render_ctx.size.width as f32 * 2.0;
		// set uniforms
		{
			let buf = render_ctx
				.device
				.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: [b0.0, b0.1, b1.0, b1.1].as_bytes(),
					usage: wgpu::BufferUsages::COPY_SRC,
				});

			render_ctx
				.encoder
				.copy_buffer_to_buffer(&buf, 0, &self.u_buf, 0, UNIFORM_SIZE);
		}

		let mut rpass = render_ctx
			.encoder
			.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[wgpu::RenderPassColorAttachment {
					view: &render_ctx.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Load,
						store: true,
					},
				}],
				depth_stencil_attachment: None,
			});

		rpass.set_pipeline(&self.pipeline);
		rpass.set_bind_group(0, &self.bind_group, &[]);
		rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
		rpass.draw(0..4, 0..1);
	}
}
