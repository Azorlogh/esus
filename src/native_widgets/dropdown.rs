use lyon::path::{builder::BorderRadii, traits::PathBuilder, Winding};

use crate::{state::State, widget::prelude::*, Color};

use super::{Button, Label};

#[derive(Debug)]
pub struct DropDown<S: State> {
	color: Color,
	button_size: Size,
	// msg: Option<S::Message>,
	child: Option<widget::Pod<S>>,
	menu: Option<widget::Pod<S>>,
}

impl<S: State> DropDown<S> {
	pub fn new(child: impl Widget<S = S> + 'static) -> Self {
		let mut s = Self::empty();
		s.child = Some(widget::Pod::new(child));
		s
	}

	pub fn empty() -> Self {
		Self {
			child: None,
			button_size: Size::new(0.0, 0.0),
			color: Color([150.0 / 255.0, 230.0 / 255.0, 100.0 / 255.0, 1.0]),
			menu: None,
		}
	}

	pub fn with_color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}

	// pub fn on_click(mut self, msg: S::Message) -> Self {
	// 	self.msg = Some(msg);
	// 	self
	// }
}

impl<S: State + 'static> Widget for DropDown<S> {
	type S = S;

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		if let Some(child) = &mut self.child {
			let size = child.size(ctx);
			self.button_size = size;
			size
		} else {
			ctx.sc.max
		}
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<Self::S>) -> Layout {
		let mut rect = ctx.suggestion.rect;
		if let Some(child) = &mut self.child {
			let mut ctx = LayoutCtx::new(ctx.state, ctx.suggestion);
			child.layout(&mut ctx);
			let child_layout = child.layout.unwrap();
			rect = child_layout.rect;
			if let Some(menu) = &mut self.menu {
				let mut menu_size_ctx = SizeCtx {
					state: ctx.state,
					sc: SizeConstraints {
						min: Size::new(0.0, 0.0),
						max: Size::new(200.0, 400.0),
					},
				};
				let menu_size = menu.size(&mut menu_size_ctx);
				let menu_suggestion = Layout {
					rect: Rect::new(
						ctx.suggestion.rect.origin + Vector::new(0.0, self.button_size.height),
						menu_size,
					),
					depth: ctx.suggestion.depth + 1.0,
				};
				let mut ctx = LayoutCtx::new(ctx.state, menu_suggestion);
				menu.layout(&mut ctx);
				rect = rect.union(&menu.layout.unwrap().rect);
			}
		}

		Layout {
			rect: rect,
			depth: ctx.suggestion.depth,
		}
	}

	fn event(&mut self, ctx: &mut EventCtx<S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				// if let Some(msg) = &self.msg {
				// 	ctx.send(msg.clone());
				// }
				match &mut self.menu {
					Some(menu) => {
						// let mut menu_ctx = EventCtx::new(ctx.state, Event::MouseDown(ctx.event));
						// menu.event(&mut menu_ctx);
						self.menu = None;
					}
					None => {
						self.menu = Some(widget::Pod::new(
							Button::empty().with_color(Color([1.0, 0.0, 1.0, 1.0])),
						));
					}
				}
				ctx.request_redraw();
			}
			_ => {}
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		if let Some(child) = &mut self.child {
			child.paint(ctx);
		}
		if let Some(menu) = &mut self.menu {
			menu.paint(ctx);
		}
	}
}
