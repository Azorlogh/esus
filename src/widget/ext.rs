use super::Widget;
use crate::{native_widgets::*, state::State};

pub trait WidgetExt<S: State>: Widget<S = S> + Sized + 'static {
	fn fix_width(self, width: f32) -> SizedBox<S> {
		SizedBox::new(self).fix_width(width)
	}

	fn fix_height(self, height: f32) -> SizedBox<S> {
		SizedBox::new(self).fix_height(height)
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
