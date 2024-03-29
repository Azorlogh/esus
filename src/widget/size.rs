use crate::{painter::Painter, Rect, Size, SizeConstraints};

#[derive(Debug)]
pub struct SizeCtx<'a, S> {
	pub state: &'a S,
	pub sc: SizeConstraints,
	pub(crate) painter: &'a mut Painter,
}

impl<'a, S> SizeCtx<'a, S> {
	pub fn new(state: &'a S, sc: SizeConstraints, painter: &'a mut Painter) -> SizeCtx<'a, S> {
		SizeCtx { state, sc, painter }
	}

	pub fn clone_with_size_constraints<'b>(&'b mut self, sc: SizeConstraints) -> SizeCtx<'b, S> {
		SizeCtx {
			state: self.state,
			sc,
			painter: self.painter,
		}
	}

	pub fn measure_text(&mut self, rect: Rect, text: &str) -> Size {
		self.painter.measure_text(rect, text)
	}
}
