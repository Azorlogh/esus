use std::collections::HashMap;

use winit::event::{MouseButton, ScanCode};

use crate::Point;

pub struct MouseState {
	pub buttons: HashMap<MouseButton, bool>,
	pub pos: Point,
}

impl Default for MouseState {
	fn default() -> MouseState {
		let mut buttons = HashMap::new();
		buttons.insert(MouseButton::Left, false);
		buttons.insert(MouseButton::Right, false);
		buttons.insert(MouseButton::Middle, false);
		MouseState {
			buttons,
			pos: Point::new(0.0, 0.0),
		}
	}
}

#[derive(Default)]
pub struct KeyboardState {
	pub keys: HashMap<ScanCode, bool>,
}

#[derive(Default)]
pub struct DeviceStates {
	pub mouse: MouseState,
	pub keyboard: KeyboardState,
}
