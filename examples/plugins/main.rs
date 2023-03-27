use esus::{
	instance,
	native_widgets::{dynamic::DynamicState, Button, DropDown, Flex, Label, LabelText},
	widget::WidgetExt,
	Color,
};

#[derive(Debug)]
struct State {
	plugin_state: DynamicState,
}

#[derive(Clone, Debug)]
enum Message {
	SetPlugin(),
}

impl esus::State for State {
	type Message = Message;
}

fn main() {
	esus::init_simple_logger();

	let mut instance = instance::Builder::<State>::new()
		.with_size((350.0, 500.0))
		.with_title("Epic test app")
		.with_state(State {
			count: 0,
			count2: 0,
		})
		.with_updater(|state, msg| match msg {
			Message::Increment => state.count += 1,
			Message::Decrement => state.count -= 1,
			Message::Increment2 => state.count2 += 1,
			Message::Decrement2 => state.count2 -= 1,
		})
		.with_view({
			Flex::column()
				.with_flex_child(
					1.0,
					Flex::row().with_child(DropDown::new(Button::new(Label::new("aaa")), |_| {
						Flex::column()
							.with_child(
								Button::new(Label::new("increment")).on_click(Message::Increment),
							)
							.with_child(Label::new("option 2"))
							.with_child(Label::new("option 3"))
					})),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, Button::empty().on_click(Message::Increment))
						.with_flex_child(
							1.0,
							Label::new(LabelText::Dynamic(Box::new(|s: &State| {
								format!("{}", s.count)
							}))),
						)
						.with_flex_child(
							1.0,
							Button::empty()
								.on_click(Message::Decrement)
								.fix_height(100.0),
						),
				)
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(
							1.0,
							Button::empty()
								.on_click(Message::Increment2)
								.with_color(Color([1.0, 0.3, 1.0, 1.0])),
						)
						.with_flex_child(
							1.0,
							Label::new(LabelText::Dynamic(Box::new(|s: &State| {
								format!("{}", s.count2)
							}))),
						)
						.with_flex_child(
							1.0,
							Button::empty()
								.on_click(Message::Decrement2)
								.with_color(Color([1.0, 0.0, 1.0, 1.0])),
						),
				)
		})
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}
