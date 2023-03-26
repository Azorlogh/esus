use super::Widget;
use crate::{native_widgets::*, state::State};

pub trait WidgetExt<'a, S: State + 'a>: Widget<S = S> + Sized + 'static {
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

	// fn adapt<SP: State>(
	// 	self,
	// 	from_state: impl for<'p> Fn(&'p SP) -> S + 'static,
	// 	to_message: impl Fn(S::Message) -> SP::Message + 'static,
	// ) -> Adapter<S, SP> {
	// 	Adapter::new(self, from_state, to_message)
	// }
}

impl<'a, S: State + 'a, W: Widget<S = S> + 'static> WidgetExt<'a, S> for W {}

// struct Foo;

// impl<S: State> Foo {
// pub fn fix_width(self, width: f64) -> W {
// 	SizedBox::new(self).fix_width(width)
// }

// pub fn fix_height(self, height: f64) -> W {
// 	SizedBox::new(self).fix_height(height)
// }
// }
