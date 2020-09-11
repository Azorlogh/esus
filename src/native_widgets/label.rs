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

pub struct Label<S> {
	text: LabelText<S>,
	id: Id,
}

impl<S> Label<S> {
	pub fn new<M>(ctx: &mut ViewCtx<S, M>, text: impl Into<LabelText<S>>) -> Label<S> {
		Label {
			id: ctx.acquire_id(),
			text: text.into(),
		}
	}
}

impl<S, M: Clone> Widget<S, M> for Label<S> {
	fn id(&self) -> Id {
		self.id
	}

	fn size(&mut self, ctx: &mut SizeCtx<S, M>) -> Size {
		ctx.sc.max
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) -> Layout {
		ctx.suggestion
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		let pos = ctx.layout().rect.origin();
		ctx.print(pos, &self.text.resolve(ctx.state));
	}
}
