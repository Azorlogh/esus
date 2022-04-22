use winit::{dpi::PhysicalSize, window::Window};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
	pub pos: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

pub struct Renderer {
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub surface: wgpu::Surface,
	pub surface_cfg: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,
	pub staging_belt: wgpu::util::StagingBelt,
	pub local_pool: futures::executor::LocalPool,
	pub local_spawner: futures::executor::LocalSpawner,
	pub depth_view: wgpu::TextureView,
}

impl Renderer {
	pub async fn new(window: &Window) -> Renderer {
		let size = window.inner_size();
		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(&wgpu::DeviceDescriptor::default(), None)
			.await
			.expect("Failed to create device");

		let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

		let surface_cfg = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: swapchain_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
		};

		surface.configure(&device, &surface_cfg);

		let depth_view = create_depth_texture(&device, size);

		let staging_belt = wgpu::util::StagingBelt::new(1024);
		let local_pool = futures::executor::LocalPool::new();
		let local_spawner = local_pool.spawner();

		Renderer {
			device,
			queue,
			surface,
			surface_cfg,
			size,
			staging_belt,
			local_pool,
			local_spawner,
			depth_view,
		}
	}

	pub fn resize(&mut self, size: PhysicalSize<u32>) {
		self.size = size;
		self.surface_cfg.width = size.width;
		self.surface_cfg.height = size.height;
		self.surface.configure(&self.device, &self.surface_cfg);
		self.depth_view = create_depth_texture(&self.device, size);
	}
}

fn create_depth_texture(device: &wgpu::Device, size: PhysicalSize<u32>) -> wgpu::TextureView {
	let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
		label: Some("Depth buffer"),
		size: wgpu::Extent3d {
			width: size.width,
			height: size.height,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Depth32Float,
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	});

	depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn next_frame<'a>(
	device: &'a mut wgpu::Device,
	surface: &'a mut wgpu::Surface,
	depth_view: &'a wgpu::TextureView,
	size: winit::dpi::PhysicalSize<u32>,
	staging_belt: &'a mut wgpu::util::StagingBelt,
	local_pool: &'a mut futures::executor::LocalPool,
	local_spawner: &'a mut futures::executor::LocalSpawner,
) -> RenderCtx<'a> {
	let mut encoder =
		device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

	let surface_tex = surface
		.get_current_texture()
		.expect("Failed to acquire next swap chain texture");

	let view = surface_tex
		.texture
		.create_view(&wgpu::TextureViewDescriptor::default());

	// clear
	encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: None,
		color_attachments: &[wgpu::RenderPassColorAttachment {
			view: &view,
			resolve_target: None,
			ops: wgpu::Operations {
				load: wgpu::LoadOp::Clear(wgpu::Color {
					r: 236.0 / 255.0,
					g: 239.0 / 255.0,
					b: 244.0 / 255.0,
					a: 1.0,
				}),
				store: true,
			},
		}],
		depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
			view: depth_view,
			depth_ops: Some(wgpu::Operations {
				load: wgpu::LoadOp::Clear(1.0),
				store: true,
			}),
			stencil_ops: None,
		}),
	});

	RenderCtx {
		device,
		encoder,
		surface_tex,
		view,
		depth_view,
		size,
		staging_belt,
		local_pool,
		local_spawner,
	}
}

pub fn finish_frame(queue: &mut wgpu::Queue, ctx: RenderCtx) {
	ctx.staging_belt.finish();
	queue.submit(Some(ctx.encoder.finish()));
	ctx.surface_tex.present();
	use futures::task::SpawnExt;
	ctx.local_spawner
		.spawn(ctx.staging_belt.recall())
		.expect("Recall staging belt");
	ctx.local_pool.run_until_stalled();
	// local_pool = ctx
}

pub struct RenderCtx<'a> {
	pub device: &'a wgpu::Device,
	pub encoder: wgpu::CommandEncoder,
	pub surface_tex: wgpu::SurfaceTexture,
	pub view: wgpu::TextureView,
	pub depth_view: &'a wgpu::TextureView,
	pub size: winit::dpi::PhysicalSize<u32>,
	pub staging_belt: &'a mut wgpu::util::StagingBelt,
	local_pool: &'a mut futures::executor::LocalPool,
	local_spawner: &'a mut futures::executor::LocalSpawner,
}
