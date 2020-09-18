use super::{EventCtx, Id, Layout, LayoutCtx, PaintCtx, Size, SizeCtx, Widget};
use pl_lens::Lens;
use std::collections::HashSet;

pub struct Pod<S, D, M> {
	pub inner: Box<dyn Widget<Box<D>, M>>,
	pub parent: Option<Id>,
	pub children: HashSet<Id>,
	pub is_active: bool,
	pub layout: Option<Layout>,
	pub lens: Box<dyn Lens<Source = S, Target = D>>,
}

impl<S, D, M> Pod<S, D, M> {
	pub fn new(
		parent: Option<Id>,
		widget: Box<dyn Widget<Box<D>, M>>,
		lens: impl Lens<Source = S, Target = D>,
	) -> Pod<S, D, M> {
		Pod {
			inner: widget,
			parent,
			is_active: false,
			layout: None,
			children: HashSet::new(),
			lens,
		}
	}
}

impl<S, D, M> Pod<S, D, M> {
	pub fn event(&mut self, ctx: &mut EventCtx<S, M>) {
		self.inner.event(ctx)
	}

	pub fn size(&mut self, ctx: &mut SizeCtx<S, M>) -> Size {
		self.inner.size(ctx)
	}

	pub fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) {
		self.layout = Some(self.inner.layout(ctx));
	}

	pub fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		self.inner.paint(ctx)
	}
}
