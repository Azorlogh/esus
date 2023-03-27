use super::Widget;
use crate::{
	native_widgets::{dynamic::DynamicState, *},
	state::{AnyState, State},
};

pub trait WidgetExt<S: State>: Widget<S = S> + Sized + 'static {
	fn fix_width(self, width: f32) -> SizedBox<S> {
		SizedBox::new(self).fix_width(width)
	}

	fn fix_height(self, height: f32) -> SizedBox<S> {
		SizedBox::new(self).fix_height(height)
	}

	fn expand_width(self) -> SizedBox<S> {
		SizedBox::new(self).expand_width()
	}

	fn expand_height(self) -> SizedBox<S> {
		SizedBox::new(self).expand_height()
	}

	fn expand(self) -> SizedBox<S> {
		SizedBox::new(self).expand()
	}

	fn with_padding(self, padding: f32) -> SizedBox<S> {
		SizedBox::new(self).with_padding(padding)
	}

	fn adapt<SP: State>(
		self,
		from_state: impl Fn(&SP) -> S + 'static,
		to_message: impl Fn(S::Message) -> SP::Message + 'static,
	) -> Adapter<S, SP> {
		Adapter::new(self, from_state, to_message)
	}

	fn dynamic(self) -> Adapter<S, DynamicState> {
		Adapter::new(self, |s: &DynamicState| s.state.downcast_ref().unwrap())
	}
}

impl<S: State, W: Widget<S = S> + 'static> WidgetExt<S> for W {}

// struct Foo;

// impl<S: State> Foo {
// pub fn fix_width(self, width: f64) -> W {
// 	SizedBox::new(self).fix_width(width)
// }

// pub fn fix_height(self, height: f64) -> W {
// 	SizedBox::new(self).fix_height(height)
// }
// }
