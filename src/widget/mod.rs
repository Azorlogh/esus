use crate::{state::State, Layout, Size};

pub mod id;
pub use id::Id;

// pub mod view;
// pub use view::ViewCtx;

pub mod event;
pub use event::EventCtx;

pub mod paint;
pub use paint::PaintCtx;

pub mod size;
pub use size::SizeCtx;

pub mod layout;
pub use layout::LayoutCtx;

pub mod hit;
pub use hit::HitCtx;

pub mod pod;
pub use pod::Pod;

mod ext;
pub use ext::WidgetExt;

pub trait Widget: std::fmt::Debug {
	type S: State;

	/// Reports whether or not a point lies inside the widget's surface,
	/// and if it does, what the depth of the surface is
	/// A higher depth means the surface "pops out the screen"
	/// e.g. a dropdown menu will have a higher depth than the surrounding content
	/// as it lies above the rest of the content
	fn hit(&mut self, _ctx: &HitCtx<Self::S>) -> Option<f32> {
		log::warn!("using default hit");
		None
	}

	fn event(&mut self, _ctx: &mut EventCtx<Self::S>) {}

	/// Reports the desired size of the widget, fitting the parent's min & max constraints
	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		ctx.sc.max
	}

	/// Reports this widget's layout based on the parent's suggestion
	/// In some cases, widgets might ignore the suggestion
	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
		ctx.suggestion
	}

	// for painting
	fn paint(&mut self, _ctx: &mut PaintCtx<Self::S>) {}
}

pub mod prelude {
	pub use crate::{
		event::Event,
		widget::{self, EventCtx, Id, LayoutCtx, PaintCtx, SizeCtx, Widget},
		Align, Align2, Color, Layout, Rect, Size, SizeConstraints, Vector,
	};
}
