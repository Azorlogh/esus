use crate::widget::prelude::*;

pub enum LabelText<S> {
	/// Specific text
	Specific(String),
	/// The provided closure is called on update, and its return
	/// value is used as the text for the label.
	Dynamic(Box<dyn Fn(&S) -> String>),
}
impl<S> From<&str> for LabelText<S> {
	fn from(src: &str) -> LabelText<S> {
		LabelText::Specific(src.to_string())
	}
}

impl<S> LabelText<S> {
	pub fn resolve(&self, s: &S) -> String {
		match self {
			LabelText::Specific(s) => s.clone(),
			LabelText::Dynamic(f) => f(s),
		}
	}
}

/*
TODO when env is implemented:
Use env to get font info and get label size.
*/

pub struct Label<S> {
	text: LabelText<S>,
	size: Size,
}

impl<S: 'static> Label<S> {
	pub fn new(text: impl Into<LabelText<S>>) -> Label<S> {
		// let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../painter/Ubuntu-M.ttf"))
		// 	.expect("couldn't load font");
		let size = Size {
			width: 100.0,
			height: 20.0,
		};
		Label {
			size,
			text: text.into(),
		}
	}

	pub fn register<M>(self, ctx: &mut ViewCtx<S, M>) -> Id {
		ctx.register(self)
	}
}

impl<S, M> Widget<S, M> for Label<S> {
	fn size(&mut self, _ctx: &mut SizeCtx<S, M>) -> Size {
		self.size
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) -> Layout {
		ctx.suggestion
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		let rect = ctx.layout().rect;
		ctx.print(rect, &self.text.resolve(ctx.state));
	}
}
