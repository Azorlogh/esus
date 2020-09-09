use crate::data::Size;

#[derive(Clone, Copy)]
pub struct SizeConstraints {
	pub min: Size,
	pub max: Size,
}
