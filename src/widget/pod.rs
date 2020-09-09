use super::{EventCtx, Id, Layout, LayoutCtx, PaintCtx, Size, Widget};
use std::collections::HashSet;

pub struct Pod<S, M> {
	pub inner: Box<dyn Widget<S, M>>,
	pub parent: Option<Id>,
	pub children: HashSet<Id>,
	pub is_active: bool,
	pub layout: Option<Layout>,
}

impl<S, M> Pod<S, M> {
	pub fn new(parent: Option<Id>, widget: Box<dyn Widget<S, M>>) -> Pod<S, M> {
		Pod {
			inner: widget,
			parent,
			is_active: false,
			layout: None,
			children: HashSet::new(),
		}
	}
}

impl<S, M> Widget<S, M> for Pod<S, M> {
	fn id(&self) -> Id {
		self.inner.id()
	}

	fn event(&mut self, ctx: &mut EventCtx<S, M>) {
		self.inner.event(ctx)
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S>) -> Layout {
		self.inner.layout(ctx)
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		self.inner.paint(ctx)
	}
}
