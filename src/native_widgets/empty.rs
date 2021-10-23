use crate::{
	state::State,
	widget::{Id, Widget},
};

pub struct Empty;

impl Empty {
	pub fn new() -> Empty {
		Empty
	}
}

impl<S: State> Widget<S> for Empty {}
