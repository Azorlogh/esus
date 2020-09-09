use crate::SizeConstraints;

pub struct SizeCtx<'a, S> {
	pub state: &'a mut S,
	pub sc: SizeConstraints,
}
