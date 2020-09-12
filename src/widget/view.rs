use super::{Id, Pool, Widget};

pub struct ViewCtx<'a, S, M> {
	pool: &'a mut Pool<S, M>,
}

impl<'a, S, M> ViewCtx<'a, S, M> {
	pub fn new(pool: &'a mut Pool<S, M>) -> ViewCtx<'a, S, M> {
		ViewCtx { pool }
	}

	pub fn register(&mut self, w: impl Widget<S, M> + 'static) -> Id {
		self.pool.add_widget(w)
	}

	pub fn set_child(&mut self, parent: Id, child: Id) {
		self.pool.set_widget_child(parent, child);
	}
}
