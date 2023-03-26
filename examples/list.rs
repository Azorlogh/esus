use std::ops::Deref;
use std::{fmt::Debug, marker::PhantomData};

use esus::native_widgets::Adapter;
use esus::{
	native_widgets::{Button, Flex, Label, LabelText, List, ListMessage, ListState},
	widget::WidgetExt,
};
#[derive(Debug)]
struct State {
	list: Vec<SubState>,
}

impl State {
	pub fn new() -> Self {
		Self {
			list: vec![SubState(0), SubState(1), SubState(2)],
		}
	}
}

#[derive(Debug, Clone)]
enum Message {
	Incr(usize),
}

impl esus::State for State {
	type Message = Message;
}

#[derive(Debug)]
struct SubState(i32);

impl esus::State for SubState {
	type Message = ();
}

// fn hint<F: FnOnce(&[u64]) -> Borrow<'_>>(f: F) -> F {
// 	f
// }

macro_rules! hoc {(
    $(for<$($lt:lifetime),*>)? |$($arg_name:tt : $ArgTy:ty),* $(,)?| -> $Ret:ty $body:block
) => (
  ({
    fn __funnel__<__F> (f: __F) -> __F
    where
        __F : $(for<$($lt),*>)? Fn($($ArgTy),*) -> $Ret,
    {
        f
    }
    __funnel__
  })(|$($arg_name),*| $body)
)}

fn view() -> impl esus::widget::Widget<S = State> {
	let list = List::new(|| {
		// Label::new(LabelText::new_dynamic(|s: &State| {
		// 	s.songs[0].source.clone()
		// })),
		Button::new(Label::new(LabelText::new_dynamic(|s: &SubState| {
			s.0.to_string()
		})))
	});

	Flex::column().with_flex_child(
		1.0,
		Adapter::new(
			list,
			hoc! { |s: &State| -> std::slice::Iter<&SubState> { s.list.iter() } },
			// |m: ListMessage<()>| Message::Incr(0),
		),
	)
}

fn main() {
	let mut instance = esus::instance::Builder::<State>::new()
		.with_size((800, 600))
		.with_state(State::new())
		.with_updater(move |state, msg| match msg {
			Message::Incr(idx) => state.list[idx].0 += 1,
		})
		.with_view(view())
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}
