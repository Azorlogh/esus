use std::marker::PhantomData;

use crate::{state::State, widget::Widget};

#[derive(Debug)]
pub struct Empty<S> {
	_phandom: PhantomData<S>,
}

impl<S: State> Empty<S> {
	pub fn new() -> Empty<S> {
		Empty {
			_phandom: PhantomData,
		}
	}
}

impl<S: State> Widget for Empty<S> {
	type S = S;
}
