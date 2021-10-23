use crate::state::State;
use crate::widget::{self, SizeCtx};
use crate::{Layout, Size, SizeConstraints};

pub struct LayoutCtx<'a, S: State> {
	pub state: &'a S,
	pub suggestion: Layout,
}

impl<'a, S: State> LayoutCtx<'a, S> {
	pub fn new(state: &'a S, suggestion: Layout) -> LayoutCtx<'a, S> {
		LayoutCtx { state, suggestion }
	}
}
