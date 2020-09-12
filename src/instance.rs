use crate::{
	device::*,
	event::*,
	painter::Painter,
	render::Renderer,
	widget::{self, Id, ViewCtx, Widget},
	Size,
};
use kurbo::Point;
use std::os::raw::c_void;
// use winapi::shared::windef::HWND;
// use winit::platform::windows::WindowBuilderExtWindows
use winit::{
	dpi::PhysicalSize,
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
	window::{Window, WindowBuilder},
};

pub struct Builder<S, M> {
	parent_window: Option<*mut c_void>,
	title: String,
	size: PhysicalSize<u32>,
	state: Option<S>,
	updater: Option<Box<dyn Fn(&mut S, M)>>,
	last_id: Id,
	view: widget::Pool<S, M>,
}

impl<S, M> Builder<S, M> {
	pub fn new() -> Builder<S, M> {
		Builder {
			parent_window: None,
			title: "App Name".to_string(),
			size: (100, 100).into(),
			state: None,
			updater: None,
			last_id: Id::initial(),
			view: widget::Pool::new(),
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

	pub fn with_updater(mut self, updater: impl Fn(&mut S, M) + 'static) -> Self {
		self.updater = Some(Box::new(updater));
		self
	}

	pub fn with_view<W>(mut self, builder: impl Fn(&mut ViewCtx<S, M>) -> W + 'static) -> Self
	where
		W: Widget<S, M> + 'static,
	{
		{
			let mut ctx = ViewCtx::new(&mut self.last_id);

			let root = Box::new(builder(&mut ctx));

			self.view.set_root_widget(root);

			for msg in &ctx.pool_queue {
				match msg {
					widget::PoolMessage::AddWidget { parent, .. } => println!("{:?}", parent),
				}
			}

			for msg in ctx.pool_queue {
				self.view.handle_message(msg);
			}
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

	pub fn build(self) -> Instance<S, M> {
		let event_loop = EventLoop::new();
		let window_builder = WindowBuilder::new().with_title(self.title);
		// if let Some(handle) = self.parent_window {
		// 	window_builder = window_builder.with_parent_window(handle as HWND);
		// }
		let window = window_builder.build(&event_loop).unwrap();
		let renderer = futures::executor::block_on(Renderer::new(&window));
		let painter = Painter::new(&renderer.device);

		let state = self.state.expect("no state was provided");
		let mut view = self.view;

		let size = window.inner_size();
		view.resolve_layout(
			&state,
			Size {
				width: size.width as f64,
				height: size.height as f64,
			},
		);

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
pub struct Instance<S, M> {
	event_loop: EventLoop<()>,
	window: Window,
	renderer: Renderer,
	painter: Painter,
	state: S,
	updater: Box<dyn Fn(&mut S, M)>,
	view: widget::Pool<S, M>,
	devices: DeviceStates,
	dead: bool,
}

impl<D, M> Instance<D, M> {
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
				wevent::Event::RedrawRequested(_) => {
					view.paint(renderer, painter, state);
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
				view.event(event, state, devices, &mut messages);
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
