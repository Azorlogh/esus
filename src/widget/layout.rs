use crate::widget::{self, SizeCtx};
use crate::{Layout, Size, SizeConstraints};

pub struct LayoutCtx<'a, S, M> {
	pub state: &'a S,
	pool: &'a mut widget::Pool<S, M>,
	pub suggestion: Layout,
}

impl<'a, S, M> LayoutCtx<'a, S, M> {
	pub fn new(
		state: &'a S,
		pool: &'a mut widget::Pool<S, M>,
		suggestion: Layout,
	) -> LayoutCtx<'a, S, M> {
		LayoutCtx {
			state,
			pool,
			suggestion,
		}
	}

	pub fn get_size(&mut self, id: widget::Id, sc: SizeConstraints) -> Size {
		let mut child = self
			.pool
			.widgets
			.remove(&id)
			.expect("tried to get size of non-existent widget");
		let mut ctx = SizeCtx::new(self.state, self.pool, sc);
		let size = child.size(&mut ctx);
		self.pool.widgets.insert(id, child);
		size
	}

	pub fn set_layout(&mut self, id: widget::Id, suggestion: Layout) {
		let mut child = self
			.pool
			.widgets
			.remove(&id)
			.expect("tried to set layout of non-existent widget");
		let mut ctx = LayoutCtx {
			state: self.state,
			pool: self.pool,
			suggestion,
		};
		child.layout(&mut ctx);
		self.pool.widgets.insert(id, child);
	}
}
