use crate::{
	device::*,
	event::*,
	painter::Painter,
	render::{self, Renderer},
	state::State,
	widget::{self, Widget},
	Size,
};
use kurbo::{Point, Rect};
use std::os::raw::c_void;
// use winapi::shared::windef::HWND;
// use winit::platform::windows::WindowBuilderExtWindows
use winit::{
	dpi::PhysicalSize,
	event_loop::{ControlFlow, EventLoop},
	platform::run_return::EventLoopExtRunReturn,
	window::{Window, WindowBuilder},
};

pub struct Builder<S: State> {
	parent_window: Option<*mut c_void>,
	title: String,
	size: PhysicalSize<u32>,
	state: Option<S>,
	updater: Option<Box<dyn Fn(&mut S, S::Message)>>,
	view: Option<widget::Pod<S>>,
}

impl<S: State> Builder<S> {
	pub fn new() -> Builder<S> {
		Builder {
			parent_window: None,
			title: "App Name".to_string(),
			size: (100, 100).into(),
			state: None,
			updater: None,
			view: None,
		}
	}

	pub fn with_parent(mut self, handle: *mut c_void) -> Self {
		self.parent_window = Some(handle);
		self
	}

	pub fn with_state(mut self, state: S) -> Self {
		self.state = Some(state);
		self
	}

	pub fn with_updater(mut self, updater: impl Fn(&mut S, S::Message) + 'static) -> Self {
		self.updater = Some(Box::new(updater));
		self
	}

	pub fn with_view(mut self, builder: impl Widget<S> + 'static) -> Self {
		{
			self.view = Some(widget::Pod::new(builder));
		}
		self
	}

	pub fn with_title(mut self, title: &str) -> Self {
		self.title = title.to_string();
		self
	}

	pub fn with_size(mut self, size: PhysicalSize<u32>) -> Self {
		self.size = size;
		self
	}

	pub fn build(self) -> Instance<S> {
		let event_loop = EventLoop::new();
		let window_builder = WindowBuilder::new()
			.with_title(self.title)
			.with_resizable(false);
		// if let Some(handle) = self.parent_window {
		// 	window_builder = window_builder.with_parent_window(handle as HWND);
		// }
		let window = window_builder.build(&event_loop).unwrap();
		let renderer = futures::executor::block_on(Renderer::new(&window));
		let painter = Painter::new(&renderer);

		let mut state = self.state.expect("no state was provided");
		let mut view = self.view.unwrap();

		let size = window.inner_size();
		{
			let mut ctx = widget::LayoutCtx::new(
				&mut state,
				crate::data::Layout {
					rect: Rect::from_origin_size(
						Point::ORIGIN,
						Size {
							width: size.width as f64,
							height: size.height as f64,
						},
					),
					depth: 0.0,
				},
			);
			view.layout(&mut ctx);
		}

		Instance {
			event_loop,
			window,
			renderer,
			painter,
			state,
			updater: self.updater.unwrap_or(Box::new(|_, _| {})),
			view,
			dead: false,
			devices: DeviceStates::default(),
		}
	}
}

///

#[allow(unused)]
pub struct Instance<S: State> {
	event_loop: EventLoop<()>,
	window: Window,
	renderer: Renderer,
	painter: Painter,
	state: S,
	updater: Box<dyn Fn(&mut S, S::Message)>,
	view: widget::Pod<S>,
	devices: DeviceStates,
	dead: bool,
}

impl<S: State> Instance<S> {
	pub fn is_dead(&self) -> bool {
		self.dead
	}

	pub fn run_return(&mut self) {
		use std::collections::VecDeque;
		use winit::event as wevent;

		let renderer = &mut self.renderer;
		let painter = &mut self.painter;
		let view = &mut self.view;
		let dead = &mut self.dead;
		let devices = &mut self.devices;
		let state = &mut self.state;
		let updater = &self.updater;
		let window = &mut self.window;
		self.event_loop.run_return(|event, _, control_flow| {
			*control_flow = ControlFlow::Exit;
			let mut events = VecDeque::new();
			match event {
				wevent::Event::WindowEvent {
					event: wevent::WindowEvent::CloseRequested,
					..
				} => {
					*dead = true;
					*control_flow = ControlFlow::Exit
				}
				wevent::Event::WindowEvent {
					event: wevent::WindowEvent::Resized(size),
					..
				} => {
					renderer.resize(size);
				}
				wevent::Event::MainEventsCleared => {
					window.request_redraw();
				}
				wevent::Event::RedrawRequested(_) => {
					// pub struct PaintCtx<'a, 'r, S> {
					// 	pub render_ctx: &'a mut RenderCtx<'r>,
					// 	pub painter: &'a mut Painter,
					// 	pub state: &'a S,
					// 	pub layout: Layout,
					// }
					let mut render_ctx = render::next_frame(
						&mut renderer.device,
						&mut renderer.surface,
						renderer.size,
						&mut renderer.staging_belt,
						&mut renderer.local_pool,
						&mut renderer.local_spawner,
					);
					let mut ctx = widget::PaintCtx {
						render_ctx: &mut render_ctx,
						painter,
						state,
						layout: view.layout.unwrap(),
					};
					view.paint(&mut ctx);

					render::finish_frame(&mut renderer.queue, render_ctx)
				}
				wevent::Event::WindowEvent { event, .. } => match event {
					wevent::WindowEvent::MouseInput { state, button, .. } => match state {
						wevent::ElementState::Pressed => {
							devices.mouse.buttons.insert(button, true);
							events.push_back(Event::MouseDown(MouseDown { button }));
						}
						wevent::ElementState::Released => {
							devices.mouse.buttons.insert(button, false);
							events.push_back(Event::MouseUp(MouseUp { button }));
						}
					},
					wevent::WindowEvent::CursorMoved { position, .. } => {
						let pos = Point::new(position.x, position.y);
						let old_pos = devices.mouse.pos;
						events.push_back(Event::MouseMove(MouseMove {
							screen_pos: pos,
							screen_old_pos: old_pos,
							pos,
							old_pos,
						}));
						devices.mouse.pos = pos;
					}
					_ => {}
				},
				_ => {}
			}

			let mut messages = VecDeque::new();
			for event in events {
				let mut ctx = widget::EventCtx::new(&event, state, devices, &mut messages);
				view.event(&mut ctx);
			}
			if messages.len() > 0 {
				window.request_redraw();
			}
			for msg in messages {
				updater(state, msg);
			}
		});
	}
}
