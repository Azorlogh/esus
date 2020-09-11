use esus::{
	instance,
	native_widgets::{Button, Flex, Label, LabelText},
};

struct State {
	count: isize,
}

#[derive(Clone)]
enum Message {
	Increment,
	Decrement,
}

fn main() {
	esus::init_simple_logger();

	let mut instance = instance::Builder::<State, Message>::new()
		.with_title("Epic test app")
		.with_state(State { count: 0 })
		.with_updater(|state, msg| match msg {
			Message::Increment => state.count += 1,
			Message::Decrement => state.count -= 1,
		})
		.with_view(|ctx| {
			let b0 = Button::new(ctx).on_click(Message::Increment);
			let label = Label::new(
				ctx,
				LabelText::Dynamic(Box::new(|s: &State| format!("{}", s.count))),
			);
			let b1 = Button::new(ctx).on_click(Message::Decrement);
			Flex::column(ctx)
				.with_child(ctx, b0)
				.with_child(ctx, label)
				.with_child(ctx, b1)
		})
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}
