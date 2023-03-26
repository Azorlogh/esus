// use esus::{
// 	instance,
// 	native_widgets::{Button, Checkbox, DropDown, Flex, Label, LabelText, SizedBox},
// 	widget::WidgetExt,
// 	Color,
// };

// #[derive(Debug, Default)]
// struct State {
// 	a: bool,
// 	b: bool,
// 	c: bool,
// }

// #[derive(Clone, Debug)]
// enum Message {
// 	SetA(bool),
// 	SetB(bool),
// 	SetC(bool),
// }

// impl esus::State for State {
// 	type Message = Message;
// }

fn main() {
	// 	esus::init_simple_logger();

	// 	let mut instance = instance::Builder::<State>::new()
	// 		.with_size((350.0, 500.0))
	// 		.with_title("Epic test app")
	// 		.with_state(State::default())
	// 		.with_updater(|state, msg| match msg {
	// 			Message::SetA(v) => state.a = v,
	// 			Message::SetB(v) => state.b = v,
	// 			Message::SetC(v) => state.c = v,
	// 		})
	// 		.with_view({
	// 			Flex::column()
	// 				.with_flex_child(
	// 					1.0,
	// 					Checkbox::new().adapt(|state: &State| state.a, |msg| Message::SetA(msg.0)),
	// 				)
	// 				.with_flex_child(
	// 					1.0,
	// 					Checkbox::new().adapt(|state: &State| state.b, |msg| Message::SetB(msg.0)),
	// 				)
	// 				.with_flex_child(
	// 					1.0,
	// 					Checkbox::new().adapt(|state: &State| state.c, |msg| Message::SetC(msg.0)),
	// 				)
	// 		})
	// 		.build();

	// 	while !instance.is_dead() {
	// 		instance.run_return();
	// 	}
}
