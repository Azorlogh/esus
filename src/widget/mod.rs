use crate::{Layout, Size};

pub mod id;
pub use id::Id;

pub mod view;
pub use view::ViewCtx;

pub mod event;
pub use event::EventCtx;

pub mod paint;
pub use paint::PaintCtx;

pub mod size;
pub use size::SizeCtx;

pub mod layout;
pub use layout::LayoutCtx;

pub mod pod;
pub use pod::Pod;

pub mod pool;
pub use pool::{Pool, PoolMessage};

mod ext;
// pub use ext::WidgetExt;

pub trait Widget<S, M> {
	// for event handling
	fn event(&mut self, _ctx: &mut EventCtx<S, M>) {}

	fn size(&mut self, ctx: &mut SizeCtx<S, M>) -> Size {
		ctx.sc.max
	}

	// to inform the instance of this widget's Layout
	fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) -> Layout {
		ctx.suggestion
	}

	// for painting
	fn paint(&mut self, _ctx: &mut PaintCtx<S>) {}
}

pub mod prelude {
	pub use crate::{
		event::Event,
		widget::{self, EventCtx, Id, LayoutCtx, PaintCtx, SizeCtx, ViewCtx, Widget},
		Layout, Rect, Size,
	};
}
