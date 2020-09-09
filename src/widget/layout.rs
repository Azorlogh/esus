use crate::data::SizeConstraints;
use kurbo::Rect;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
	pub rect: Rect,
	pub depth: f64,
}

pub struct LayoutCtx<'a, S> {
	pub state: &'a S,
	pub suggested: Layout,
}

impl<'a, S> LayoutCtx<'a, S> {
	pub fn new(state: &'a S, suggested: Layout) -> LayoutCtx<'a, S> {
		LayoutCtx { state, suggested }
	}
}
