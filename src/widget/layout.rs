use super::SizeCtx;
use crate::{painter::Painter, state::State, Layout, SizeConstraints};

pub struct LayoutCtx<'a, S: State> {
	pub state: &'a S,
	pub suggestion: Layout,
	pub(crate) painter: &'a mut Painter,
}

impl<'a, S: State> LayoutCtx<'a, S> {
	pub fn new(state: &'a S, suggestion: Layout, painter: &'a mut Painter) -> LayoutCtx<'a, S> {
		LayoutCtx {
			state,
			suggestion,
			painter,
		}
	}

	pub fn clone_with_layout<'b>(&'b mut self, layout: Layout) -> LayoutCtx<'b, S> {
		LayoutCtx {
			state: self.state,
			suggestion: layout,
			painter: self.painter,
		}
	}

	pub fn create_size_context<'b>(&'b mut self, sc: SizeConstraints) -> SizeCtx<'b, S> {
		SizeCtx::new(self.state, sc, self.painter)
	}
}
