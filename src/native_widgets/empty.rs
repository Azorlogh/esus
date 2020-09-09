use crate::widget::{Id, ViewCtx, Widget};

pub struct Empty {
	id: Id,
}

impl Empty {
	pub fn new<'a, S, M>(ctx: &mut ViewCtx<'a, S, M>) -> Empty {
		Empty {
			id: ctx.acquire_id(),
		}
	}
}

impl<S, M> Widget<S, M> for Empty {
	fn id(&self) -> Id {
		self.id
	}
}
