#![allow(unused)]

#[derive(Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

impl Axis {
	pub fn major<T, V: Into<(T, T)>>(&self, coords: V) -> T {
		match *self {
			Axis::X => coords.into().0,
			Axis::Y => coords.into().1,
		}
	}

	pub fn with_major<T, V: Into<(T, T)> + From<(T, T)>>(&self, coords: V, value: T) -> V {
		let mut t = coords.into();
		match *self {
			Axis::X => t.0 = value,
			Axis::Y => t.1 = value,
		}
		t.into()
	}
}
