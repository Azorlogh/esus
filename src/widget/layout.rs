use crate::painter::Painter;
use crate::state::State;
use crate::{Layout, SizeConstraints};

use super::SizeCtx;

pub struct LayoutCtx<'a, S: State> {
	pub state: &'a S,
	pub suggestion: Layout,
	painter: &'a mut Painter,
}

impl<'a, S: State> LayoutCtx<'a, S> {
	pub fn new(state: &'a S, suggestion: Layout, painter: &'a mut Painter) -> LayoutCtx<'a, S> {
		LayoutCtx {
			state,
			suggestion,
			painter,
		}
	}

	pub fn clone_with_layout<'b>(&'a mut self, layout: Layout) -> LayoutCtx<'b, S>
	where
		'a: 'b,
	{
		LayoutCtx {
			state: self.state,
			suggestion: layout,
			painter: self.painter,
		}
	}

	pub fn create_size_context<'b>(&'a mut self, sc: SizeConstraints) -> SizeCtx<'b, S>
	where
		'a: 'b,
	{
		SizeCtx::new(self.state, sc, self.painter)
	}
}
