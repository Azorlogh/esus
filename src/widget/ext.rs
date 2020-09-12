use super::{ViewCtx, Widget};
use crate::native_widgets::*;

pub trait WidgetExt<S, M>: Widget<S, M> + Sized + 'static {
	fn fix_width(self, ctx: &mut ViewCtx<S, M>, width: f64) -> SizedBox {
		SizedBox::new(ctx, self).fix_width(width)
	}

	fn fix_height(self, ctx: &mut ViewCtx<S, M>, height: f64) -> SizedBox {
		SizedBox::new(ctx, self).fix_height(height)
	}
}

impl<S, M, W: Widget<S, M> + 'static> WidgetExt<S, M> for W {}
