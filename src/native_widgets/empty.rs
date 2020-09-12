use crate::widget::{Id, ViewCtx, Widget};

pub struct Empty;

impl Empty {
	pub fn new() -> Empty {
		Empty
	}

	pub fn register<'a, S, M>(self, ctx: &mut ViewCtx<'a, S, M>) -> Id {
		ctx.register(self)
	}
}

impl<S, M> Widget<S, M> for Empty {}
