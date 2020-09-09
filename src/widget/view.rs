use super::{Id, PoolMessage, Widget};
use std::collections::VecDeque;

pub struct ViewCtx<'a, S, M> {
	pub pool_queue: VecDeque<PoolMessage<S, M>>,
	last_id: &'a mut Id,
}

impl<'a, S, M> ViewCtx<'a, S, M> {
	pub fn new(last_id: &'a mut Id) -> ViewCtx<'a, S, M> {
		ViewCtx {
			pool_queue: VecDeque::new(),
			last_id,
		}
	}

	pub fn acquire_id(&mut self) -> Id {
		self.last_id.next()
	}

	pub fn add_widget(&mut self, parent: Id, w: impl Widget<S, M> + 'static) -> Id {
		let id = w.id();
		self.pool_queue.push_back(PoolMessage::AddWidget {
			parent,
			widget: Box::new(w),
		});
		id
	}
}
