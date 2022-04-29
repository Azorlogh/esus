use crate::{Layout, Point};

#[derive(Debug)]
pub struct HitCtx<'a, S> {
	pub state: &'a S,
	pub layout: Layout,
	pub point: Point,
}

impl<'a, S> HitCtx<'a, S> {
	pub fn new(state: &'a S, layout: Layout, point: Point) -> Self {
		Self {
			state,
			layout,
			point,
		}
	}

	pub fn clone_with_layout(&self, layout: Layout) -> Self {
		Self {
			state: self.state,
			layout,
			point: self.point,
		}
	}
}
