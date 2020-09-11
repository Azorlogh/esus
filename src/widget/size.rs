use crate::widget;
use crate::{Size, SizeConstraints};

pub struct SizeCtx<'a, S, M> {
	pub state: &'a S,
	pool: &'a mut widget::Pool<S, M>,
	pub sc: SizeConstraints,
}

impl<'a, S, M> SizeCtx<'a, S, M> {
	pub fn new(
		state: &'a S,
		pool: &'a mut widget::Pool<S, M>,
		sc: SizeConstraints,
	) -> SizeCtx<'a, S, M> {
		SizeCtx { state, pool, sc }
	}

	pub fn get_size(&mut self, id: widget::Id, sc: SizeConstraints) -> Size {
		let mut child = self
			.pool
			.widgets
			.remove(&id)
			.expect("tried to get size of non-existent widget");
		let mut ctx = SizeCtx {
			state: self.state,
			pool: self.pool,
			sc,
		};
		let size = child.size(&mut ctx);
		self.pool.widgets.insert(id, child);
		size
	}
}
