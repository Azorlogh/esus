use crate::Rect;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
	pub rect: Rect,
	pub depth: f64,
}
