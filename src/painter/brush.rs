use std::borrow::Cow;

use super::RenderCtx;
use crate::{
	render::{Renderer, Vertex},
	util, Color,
};
use lyon::{
	lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
	path::Path,
};
use wgpu::util::DeviceExt;
use zerocopy::AsBytes as _;

const UNIFORM_SIZE: wgpu::BufferAddress = 16 * std::mem::size_of::<f32>() as wgpu::BufferAddress;
const VERTEX_SIZE: wgpu::BufferAddress = 2 * std::mem::size_of::<f32>() as wgpu::BufferAddress;

#[allow(dead_code)]
pub struct Brush {
	vs_module: wgpu::ShaderModule,
	fs_module: wgpu::ShaderModule,
	transform_buf: wgpu::Buffer,
	color_buf: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
	pipeline: wgpu::RenderPipeline,

	transform: cgmath::Matrix4<f32>,
}

impl Brush {
	pub fn new(renderer: &Renderer) -> Self {
		let vs_module = renderer
			.device
			.create_shader_module(&wgpu::ShaderModuleDescriptor {
				label: None,
				source: wgpu::ShaderSource::SpirV(Cow::Owned(util::compile_shader(
					include_str!("brush.vert"),
					shaderc::ShaderKind::Vertex,
				))),
			});
		let fs_module = renderer
			.device
			.create_shader_module(&wgpu::ShaderModuleDescriptor {
				label: None,
				source: wgpu::ShaderSource::SpirV(Cow::Owned(util::compile_shader(
					include_str!("brush.frag"),
					shaderc::ShaderKind::Fragment,
				))),
			});

		let transform = cgmath::Matrix4::new(
			2.0 / renderer.surface_cfg.width as f32,
			0.0,
			0.0,
			0.0,
			0.0,
			-2.0 / renderer.surface_cfg.height as f32,
			0.0,
			0.0,
			0.0,
			0.0,
			0.0,
			0.0,
			-1.0,
			1.0,
			0.0,
			1.0,
		);

		let tr_buf = renderer
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(&transform)),
				usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
			});

		let col_buf = renderer
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&[1f32, 0.0, 0.0, 1.0]),
				usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
			});

		let bind_group_layout =
			renderer
				.device
				.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
					entries: &[
						wgpu::BindGroupLayoutEntry {
							binding: 0,
							visibility: wgpu::ShaderStages::VERTEX,
							ty: wgpu::BindingType::Buffer {
								ty: wgpu::BufferBindingType::Uniform,
								has_dynamic_offset: false,
								min_binding_size: None,
							},
							count: None,
						},
						wgpu::BindGroupLayoutEntry {
							binding: 1,
							visibility: wgpu::ShaderStages::FRAGMENT,
							ty: wgpu::BindingType::Buffer {
								ty: wgpu::BufferBindingType::Uniform,
								has_dynamic_offset: false,
								min_binding_size: None,
							},
							count: None,
						},
					],
					label: None,
				});

		let bind_group = renderer
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &bind_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: tr_buf.as_entire_binding(),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: col_buf.as_entire_binding(),
					},
				],
				label: None,
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

		Self {
			vs_module,
			fs_module,
			transform_buf: tr_buf,
			color_buf: col_buf,
			bind_group,
			pipeline,
			transform,
		}
	}

	pub fn set_color(&mut self, render_ctx: &mut RenderCtx, col: Color) {
		{
			let buf = render_ctx
				.device
				.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(&col.0),
					usage: wgpu::BufferUsages::COPY_SRC,
				});

			render_ctx.encoder.copy_buffer_to_buffer(
				&buf,
				0,
				&self.color_buf,
				0,
				4 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
			);
		}
	}

	pub fn fill(&mut self, render_ctx: &mut RenderCtx, path: &Path) {
		{
			let buf = render_ctx
				.device
				.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(&self.transform)),
					usage: wgpu::BufferUsages::COPY_SRC,
				});

			render_ctx
				.encoder
				.copy_buffer_to_buffer(&buf, 0, &self.transform_buf, 0, UNIFORM_SIZE);
		}

		let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
		let mut tesselator = FillTessellator::new();
		tesselator
			.tessellate_path(
				path,
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vertex {
					pos: vertex.position().to_array(),
				}),
			)
			.unwrap(); // TODO: remove this

		let vbo = render_ctx
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&geometry.vertices),
				usage: wgpu::BufferUsages::VERTEX,
			});

		let ibo = render_ctx
			.device
			.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(&geometry.indices),
				usage: wgpu::BufferUsages::INDEX,
			});

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
		rpass.set_vertex_buffer(0, vbo.slice(..));
		rpass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint16);
		rpass.draw_indexed(0..geometry.indices.len() as u32, 0, 0..1);
	}
}
