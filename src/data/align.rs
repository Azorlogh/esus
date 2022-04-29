#[derive(Clone, Copy, Debug, Default)]
pub struct Align2 {
	pub x: Align,
	pub y: Align,
}

#[derive(Clone, Copy, Debug)]
pub enum Align {
	Min,
	Center,
	Max,
}

impl Default for Align {
	fn default() -> Self {
		Align::Center
	}
}
