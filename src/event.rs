use winit::event::MouseButton;

use crate::{Point, Vector};

#[derive(Debug, Clone)]
pub struct MouseDown {
	pub button: MouseButton,
}

#[derive(Debug, Clone)]
pub struct MouseUp {
	pub button: MouseButton,
}

#[derive(Debug, Clone)]
pub struct MouseMove {
	pub screen_pos: Point,
	pub screen_old_pos: Point,
	pub pos: Point,
	pub old_pos: Point,
}

impl MouseMove {
	pub fn delta(&self) -> Vector {
		self.pos.to_vector() - self.old_pos.to_vector()
	}
}

#[derive(Debug, Clone)]
pub enum Event {
	Update,
	MouseDown(MouseDown),
	MouseUp(MouseUp),
	MouseMove(MouseMove),
}
