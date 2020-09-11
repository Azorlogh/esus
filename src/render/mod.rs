use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub surface: wgpu::Surface,
	pub sc_desc: wgpu::SwapChainDescriptor,
	pub swapchain: wgpu::SwapChain,
	pub size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
	pub async fn new(window: &Window) -> Renderer {
		let size = window.inner_size();
		let surface = wgpu::Surface::create(window);
		let adapter = wgpu::Adapter::request(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::Default,
				compatible_surface: Some(&surface),
			},
			wgpu::BackendBit::PRIMARY,
		)
		.await
		.unwrap();

		let (device, queue) = adapter
			.request_device(&wgpu::DeviceDescriptor {
				extensions: wgpu::Extensions {
					anisotropic_filtering: false,
				},
				limits: wgpu::Limits::default(),
			})
			.await;

		let sc_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Mailbox,
		};

		let swapchain = device.create_swap_chain(&surface, &sc_desc);

		Renderer {
			device,
			queue,
			surface,
			sc_desc,
			swapchain,
			size,
		}
	}

	pub fn resize(&mut self, size: PhysicalSize<u32>) {
		self.sc_desc.width = size.width;
		self.sc_desc.height = size.height;
		self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
	}
}

pub fn next_frame<'a>(
	device: &'a mut wgpu::Device,
	sc: &'a mut wgpu::SwapChain,
	size: winit::dpi::PhysicalSize<u32>,
) -> RenderCtx<'a> {
	let mut encoder =
		device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

	let frame = sc
		.get_next_texture()
		.expect("Error fetching swapchain texture");

	// clear
	encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
			attachment: &frame.view,
			resolve_target: None,
			load_op: wgpu::LoadOp::Clear,
			store_op: wgpu::StoreOp::Store,
			clear_color: wgpu::Color {
				r: 0.0,
				g: 0.05,
				b: 0.1,
				a: 1.0,
			},
		}],
		depth_stencil_attachment: None,
	});

	RenderCtx {
		device,
		encoder,
		frame,
		size,
	}
}

pub fn finish_frame(queue: &mut wgpu::Queue, ctx: RenderCtx) {
	queue.submit(&[ctx.encoder.finish()]);
}

pub struct RenderCtx<'a> {
	pub device: &'a wgpu::Device,
	pub encoder: wgpu::CommandEncoder,
	pub frame: wgpu::SwapChainOutput,
	pub size: winit::dpi::PhysicalSize<u32>,
}
