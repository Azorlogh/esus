use super::{Id, Widget};

pub struct ViewCtx<'a, S> {
	pool: &'a mut Pool<S>,
}

impl<'a, S: State> ViewCtx<'a, S> {
	pub fn new(pool: &'a mut Pool<S>) -> ViewCtx<'a, S> {
		ViewCtx { pool }
	}

	pub fn register(&mut self, w: impl Widget<S> + 'static) -> Id {
		self.pool.add_widget(w)
	}

	pub fn set_child(&mut self, parent: Id, child: Id) {
		self.pool.set_widget_child(parent, child);
	}
}
