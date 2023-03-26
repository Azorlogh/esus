use std::iter::once;

use custom_debug::Debug;

use crate::{
	state::State,
	widget::{prelude::*, HitCtx},
};

#[derive(Debug)]
pub struct DropDown<S: State> {
	child: widget::Pod<S>,
	#[debug(skip)]
	menu_builder: Box<dyn Fn(&EventCtx<S>) -> widget::Pod<S> + 'static>,
	menu: Option<widget::Pod<S>>,
}

impl<S: State> DropDown<S> {
	pub fn new<B, M>(child: impl Widget<S = S> + 'static, menu_builder: B) -> Self
	where
		B: Fn(&EventCtx<S>) -> M + 'static,
		M: Widget<S = S> + 'static,
	{
		Self {
			child: widget::Pod::new(child),
			menu_builder: Box::new(move |ctx| widget::Pod::new(menu_builder(ctx))),
			menu: None,
		}
	}
}

impl<S: State + 'static> Widget for DropDown<S> {
	type S = S;

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		self.child.size(ctx)
	}

	fn layout(&mut self, mut ctx: LayoutCtx<Self::S>) -> Layout {
		self.child.layout(ctx.clone_with_layout(ctx.suggestion));
		let child_layout = self.child.layout.unwrap();
		let mut rect = child_layout.rect;
		if let Some(menu) = &mut self.menu {
			let mut menu_size_ctx = ctx.create_size_context(SizeConstraints {
				min: Size::new(0.0, 0.0),
				max: Size::new(200.0, 400.0),
			});
			let menu_size = menu.size(&mut menu_size_ctx);
			let menu_suggestion = Layout {
				rect: Rect::new(
					ctx.suggestion.rect.origin + Vector::new(0.0, child_layout.rect.height()),
					menu_size,
				),
				depth: ctx.suggestion.depth + 1.0,
			};
			menu.layout(ctx.clone_with_layout(menu_suggestion));
			rect = rect.union(&menu.layout.unwrap().rect);
		}

		Layout {
			rect,
			depth: ctx.suggestion.depth,
		}
	}

	fn hit(&mut self, ctx: &widget::HitCtx<Self::S>) -> Option<f32> {
		once(&mut self.child)
			.chain(self.menu.iter_mut())
			.map(|child| {
				let hit = child.hit(ctx);
				hit
			})
			.fold(None, |a, b| match (a, b) {
				(Some(a), Some(b)) => Some(a.max(b)),
				(Some(a), None) => Some(a),
				(None, Some(b)) => Some(b),
				(None, None) => None,
			})
	}

	fn event(&mut self, ctx: &mut EventCtx<S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				// if let Some(msg) = &self.msg {
				// 	ctx.send(msg.clone());
				// }
				match &mut self.menu {
					Some(menu) => {
						let hit_ctx =
							HitCtx::new(ctx.state, menu.layout.unwrap(), ctx.devices.mouse.pos);
						if menu.hit(&hit_ctx).is_some() {
							// let mut menu_ctx =
							// 	EventCtx::new(ctx.state, Event::MouseDown(ctx.event));
							menu.event(ctx);
						} else {
							self.menu = None;
						}
					}
					None => {
						self.menu = Some((self.menu_builder)(ctx));
					}
				}
				ctx.request_redraw();
			}
			_ => {}
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		self.child.paint(ctx);
		if let Some(menu) = &mut self.menu {
			menu.paint(ctx);
		}
	}
}
