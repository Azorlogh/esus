use crate::WindowDesc;
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

pub struct AppLauncher<D> {
	window_desc: Option<WindowDesc<D>>,
}

impl<D> AppLauncher<D> {
	pub fn new() -> AppLauncher<D> {
		AppLauncher { window_desc: None }
	}

	pub fn with_window(mut self, desc: WindowDesc<D>) -> Self {
		self.window_desc = Some(desc);
		self
	}

	pub fn launch(mut self) {
		if let Some(window_desc) = self.window_desc {
			let event_loop = EventLoop::new();
			let window = WindowBuilder::new()
				.with_title(window_desc.title)
				.build(&event_loop)
				.unwrap();
			event_loop.run(move |event, _, control_flow| {
				*control_flow = ControlFlow::Wait;

				match event {
					Event::WindowEvent {
						event: WindowEvent::CloseRequested,
						window_id,
					} if window_id == window.id() => *control_flow = ControlFlow::Exit,
					Event::RedrawRequested(_) => {
						// draw
					}
					_ => {}
				}
			})
		}
	}
}
