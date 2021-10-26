use crate::{event::Event, state::State};

use super::{EventCtx, Layout, LayoutCtx, PaintCtx, Size, SizeCtx, Widget};

pub struct Pod<S> {
	pub inner: Box<dyn Widget<S>>,
	pub is_active: bool,
	pub layout: Option<Layout>,
}

impl<S> std::fmt::Debug for Pod<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Pod")
			.field("is_active", &self.is_active)
			.field("layout", &self.layout)
			.finish()
	}
}

impl<S: State> Pod<S> {
	pub fn new(widget: impl Widget<S> + 'static) -> Pod<S> {
		Pod {
			inner: Box::new(widget),
			is_active: false,
			layout: None,
		}
	}
}

impl<S: State> Pod<S> {
	pub fn event(&mut self, ctx: &mut EventCtx<S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				if let Some(layout) = self.layout {
					if layout.rect.contains(ctx.devices.mouse.pos) {
						self.inner.event(ctx);
					}
				}
			}
			_ => self.inner.event(ctx),
		}
	}

	pub fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		self.inner.size(ctx)
	}

	pub fn layout(&mut self, ctx: &mut LayoutCtx<S>) {
		self.layout = Some(self.inner.layout(ctx));
	}

	pub fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		if let Some(layout) = self.layout {
			ctx.layout = layout;
		}
		self.inner.paint(ctx)
	}
}
