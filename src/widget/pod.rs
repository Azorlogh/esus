use crate::{event::Event, state::State, Size};

use super::{EventCtx, HitCtx, Layout, LayoutCtx, PaintCtx, SizeCtx, Widget};

#[derive(Debug)]
pub struct Pod<S> {
	pub inner: Box<dyn Widget<S = S>>,
	pub is_active: bool,
	pub layout: Option<Layout>,
}

// impl<S> std::fmt::Debug for Pod<S> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		f.debug_struct("Pod")
// 			.field("is_active", &self.is_active)
// 			.field("layout", &self.layout)
// 			.finish()
// 	}
// }

impl<S: State> Pod<S> {
	pub fn new(widget: impl Widget<S = S> + 'static) -> Pod<S> {
		Pod {
			inner: Box::new(widget),
			is_active: false,
			layout: None,
		}
	}
}

impl<S: State> Pod<S> {
	pub fn hit(&mut self, ctx: &HitCtx<S>) -> Option<f32> {
		if let Some(layout) = self.layout {
			log::warn!("hitting pod! {layout:?}");
			if layout.rect.contains(ctx.point) {
				self.inner.hit(&ctx.clone_with_layout(layout))
			} else {
				None
			}
		} else {
			log::warn!("not hit, no layout :(");
			None
		}
	}

	pub fn event(&mut self, ctx: &mut EventCtx<S>) {
		self.inner.event(ctx);
		// match ctx.event {
		// 	Event::MouseDown(_) => {
		// 		if let Some(layout) = self.layout {
		// 			if layout.rect.contains(ctx.devices.mouse.pos) {
		// 				self.inner.event(ctx);
		// 			}
		// 		}
		// 	}
		// 	_ => self.inner.event(ctx),
		// }
	}

	pub fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		self.inner.size(ctx)
	}

	pub fn layout(&mut self, ctx: &mut LayoutCtx<S>) {
		let layout = self.inner.layout(ctx);
		self.layout = Some(layout);
	}

	pub fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		if let Some(layout) = self.layout {
			ctx.layout = layout;
			self.inner.paint(ctx)
		}
	}
}
