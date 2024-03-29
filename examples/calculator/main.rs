// WIP
// Adapted from Druid's calc example

use esus::{
	instance,
	native_widgets::{Button, Flex, Label, LabelText},
	widget::{Widget, WidgetExt},
};

// Some useful data types
#[derive(Debug, Clone)]
enum Operation {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug)]
struct State {
	value: String,
	operand: f32,
	operation: Option<Operation>,
	is_result: bool,
}

impl State {
	fn new() -> Self {
		Self {
			value: "0".to_owned(),
			operand: 0.0,
			operation: None,
			is_result: true,
		}
	}

	fn clear_entry(&mut self) {
		self.value = "0".to_owned();
		self.is_result = true;
	}

	fn clear(&mut self) {
		self.clear_entry();
		self.operand = 0.0;
		self.operation = None;
	}

	fn backspace(&mut self) {
		if !self.is_result {
			self.value.pop();
			if self.value.is_empty() || self.value == "-" {
				self.value = "0".to_string();
				self.is_result = true;
			}
		}
	}

	fn swap_sign(&mut self) {
		if !self.is_result {
			if self.value.starts_with('-') {
				self.value = self.value[1..].to_string();
			} else {
				self.value = ["-", &self.value].concat();
			}
		}
	}

	fn decimal_mark(&mut self) {
		if self.is_result {
			self.value = "0.".to_owned();
		} else {
			if !self.value.contains('.') {
				self.value += ".";
			}
		}
	}

	fn digit(&mut self, dgt: u8) {
		if self.is_result {
			self.value = format!("{}", dgt);
			self.is_result = false;
		} else {
			self.value = format!("{}{}", self.value, dgt);
		}
	}

	fn operation(&mut self, op: Operation) {
		self.operation = Some(op);
		self.operand = self.value.parse().unwrap();
		self.clear_entry();
	}

	fn evaluate(&mut self) {
		if let Some(op) = &self.operation {
			let value = self.value.parse::<f32>().unwrap();
			let result = match op {
				Operation::Add => self.operand + value,
				Operation::Sub => self.operand - value,
				Operation::Mul => self.operand * value,
				Operation::Div => self.operand / value,
			};
			self.value = format!("{}", result);
			self.is_result = true;
			self.operation = None;
		}
	}
}

#[derive(Clone, Debug)]
enum Message {
	Digit(u8),
	ClearEntry,
	Clear,
	Backspace,
	DecimalMark,
	Op(Operation),
	SwapSign,
	Equal,
}

impl esus::State for State {
	type Message = Message;
}

fn button(txt: &str, msg: Message) -> impl Widget<S = State> {
	Button::new(Label::new(txt)).on_click(msg).with_padding(1.0)
}

fn main() {
	esus::init_simple_logger();

	let mut instance = instance::Builder::<State>::new()
		.with_size((230.0, 300.0))
		.with_title("Esus Calculator Demo")
		.with_state(State::new())
		.with_updater(|state, msg| match msg {
			Message::ClearEntry => state.clear_entry(),
			Message::Clear => state.clear(),
			Message::Backspace => state.backspace(),
			Message::SwapSign => state.swap_sign(),
			Message::DecimalMark => state.decimal_mark(),
			Message::Digit(dgt) => state.digit(dgt),
			Message::Op(op) => state.operation(op),
			Message::Equal => state.evaluate(),
		})
		.with_view({
			Flex::column()
				.with_child(
					Label::new(LabelText::Dynamic(Box::new(|s: &State| {
						format!("{}", s.value)
					})))
					.fix_height(70.0),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, button("CE", Message::ClearEntry))
						.with_flex_child(1.0, button("C", Message::Clear))
						.with_flex_child(1.0, button("<×", Message::Backspace))
						.with_flex_child(1.0, button("÷", Message::Op(Operation::Div))),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, button("7", Message::Digit(7)))
						.with_flex_child(1.0, button("8", Message::Digit(8)))
						.with_flex_child(1.0, button("9", Message::Digit(9)))
						.with_flex_child(1.0, button("×", Message::Op(Operation::Mul))),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, button("4", Message::Digit(4)))
						.with_flex_child(1.0, button("5", Message::Digit(5)))
						.with_flex_child(1.0, button("6", Message::Digit(6)))
						.with_flex_child(1.0, button("-", Message::Op(Operation::Sub))),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, button("1", Message::Digit(1)))
						.with_flex_child(1.0, button("2", Message::Digit(2)))
						.with_flex_child(1.0, button("3", Message::Digit(3)))
						.with_flex_child(1.0, button("+", Message::Op(Operation::Add))),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, button("±", Message::SwapSign))
						.with_flex_child(1.0, button("0", Message::Digit(0)))
						.with_flex_child(1.0, button(".", Message::DecimalMark))
						.with_flex_child(1.0, button("=", Message::Equal)),
				)
		})
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}
