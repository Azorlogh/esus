use crate::data::Size;

#[derive(Clone, Copy, Debug)]
pub struct SizeConstraints {
	pub min: Size,
	pub max: Size,
}
