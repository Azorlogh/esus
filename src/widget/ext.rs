use super::{Id, ViewCtx};
use crate::native_widgets::*;

impl Id {
	pub fn fix_width<S, M>(self, ctx: &mut ViewCtx<S, M>, width: f64) -> Id {
		SizedBox::new(self).fix_width(width).register(ctx)
	}

	pub fn fix_height<S, M>(self, ctx: &mut ViewCtx<S, M>, height: f64) -> Id {
		SizedBox::new(self).fix_height(height).register(ctx)
	}
}
