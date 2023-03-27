use std::os::raw::c_void;

// use winapi::shared::windef::HWND;
// use winit::platform::windows::WindowBuilderExtWindows
use winit::{
	dpi::PhysicalSize,
	event_loop::{ControlFlow, EventLoop},
	platform::run_return::EventLoopExtRunReturn,
	window::{Window, WindowBuilder},
};

use crate::{
	device::*,
	event::*,
	painter::Painter,
	render::{self, Renderer},
	state::State,
	widget::{self, Widget},
	Point, Rect, Size,
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

	pub fn with_view(mut self, builder: impl Widget<S = S> + 'static) -> Self {
		{
			self.view = Some(widget::Pod::new(builder));
		}
		self
	}

	pub fn with_title(mut self, title: &str) -> Self {
		self.title = title.to_string();
		self
	}

	pub fn with_size<T>(mut self, size: T) -> Self
	where
		T: Into<PhysicalSize<u32>>,
	{
		self.size = size.into();
		self
	}

	pub fn build(self) -> Instance<S> {
		let event_loop = EventLoop::new();
		let window_builder = WindowBuilder::new()
			.with_title(self.title)
			.with_inner_size(self.size)
			.with_resizable(false);
		// if let Some(handle) = self.parent_window {
		// 	window_builder = window_builder.with_parent_window(handle as HWND);
		// }
		let window = window_builder.build(&event_loop).unwrap();
		let renderer = futures::executor::block_on(Renderer::new(&window));
		let mut painter = Painter::new(&renderer);

		let mut state = self.state.expect("no state was provided");
		let mut view = self.view.unwrap();

		let size = window.inner_size();
		{
			let ctx = widget::LayoutCtx::new(
				&mut state,
				crate::data::Layout {
					rect: Rect::from_size(Size::new(size.width as f32, size.height as f32)),
					depth: 0.0,
				},
				&mut painter,
			);
			view.layout(ctx);
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
					painter.resize(size);
					{
						let ctx = widget::LayoutCtx::new(
							state,
							crate::data::Layout {
								rect: Rect::from_size(Size::new(
									size.width as f32,
									size.height as f32,
								)),
								depth: 0.0,
							},
							painter,
						);
						view.layout(ctx);
					}
				}
				wevent::Event::MainEventsCleared => {}
				wevent::Event::RedrawRequested(_) => {
					println!("figuring out layout");
					{
						let size = window.inner_size();
						let ctx = widget::LayoutCtx::new(
							state,
							crate::data::Layout {
								rect: Rect::from_size(Size::new(
									size.width as f32,
									size.height as f32,
								)),
								depth: 0.0,
							},
							painter,
						);
						view.layout(ctx);
					}
					println!("drawing");
					let mut render_ctx = render::next_frame(
						&mut renderer.device,
						&mut renderer.surface,
						&mut renderer.depth_view,
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
						let pos = Point::new(position.x as f32, position.y as f32);
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

			// TODO: this loop is stupid, should apply the messages after each event
			let mut messages = VecDeque::new();
			for event in events {
				let mut redraw_requested = false;
				let mut ctx = widget::EventCtx::new(
					&event,
					state,
					devices,
					&mut redraw_requested,
					&mut messages,
				);
				view.event(&mut ctx);
				if redraw_requested {
					window.request_redraw();
				}
			}
			let updated = messages.len() > 0;
			if updated {
				window.request_redraw();
			}
			for msg in messages {
				updater(state, msg);
			}
			if updated {
				let mut redraw_requested = false;
				let mut messages = VecDeque::new();
				let mut ctx = widget::EventCtx::new(
					&Event::Update,
					state,
					devices,
					&mut redraw_requested,
					&mut messages,
				);
				view.event(&mut ctx);
			}
		});
	}
}
