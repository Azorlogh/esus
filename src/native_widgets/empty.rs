use crate::{state::State, widget::Widget};

pub struct Empty;

impl Empty {
	pub fn new() -> Empty {
		Empty
	}
}

impl<S: State> Widget<S> for Empty {}
