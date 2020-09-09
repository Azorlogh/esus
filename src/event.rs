use kurbo::{Point, Vec2};
use winit::event::MouseButton;

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
	pub fn delta(&self) -> Vec2 {
		self.pos.to_vec2() - self.old_pos.to_vec2()
	}
}

#[derive(Debug, Clone)]
pub enum Event {
	MouseDown(MouseDown),
	MouseUp(MouseUp),
	MouseMove(MouseMove),
}
