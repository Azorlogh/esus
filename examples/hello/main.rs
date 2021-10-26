use esus::{
	instance,
	native_widgets::{Button, Empty, Flex, Label, LabelText},
};

#[derive(Debug)]
struct State {
	count: isize,
}

#[derive(Clone)]
enum Message {
	Increment,
	Decrement,
}

impl esus::State for State {
	type Message = Message;
}

fn main() {
	esus::init_simple_logger();

	let mut instance = instance::Builder::<State>::new()
		.with_title("Epic test app")
		.with_state(State { count: 0 })
		.with_updater(|state, msg| match msg {
			Message::Increment => state.count += 1,
			Message::Decrement => state.count -= 1,
		})
		.with_view({
			let b0 = Button::new().on_click(Message::Increment);
			let label = Label::new(LabelText::Dynamic(Box::new(|s: &State| {
				format!("{}", s.count)
			})));
			// .fix_height(ctx, 100.0);
			let b1 = Button::new().on_click(Message::Decrement);
			Flex::row().with_child(b0).with_child(label).with_child(b1)

			// Button::new().on_click(Message::Increment)

			// Empty::new()
		})
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}
