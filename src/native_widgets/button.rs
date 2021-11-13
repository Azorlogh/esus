use lyon::path::{builder::BorderRadii, traits::PathBuilder, Winding};

use crate::{state::State, widget::prelude::*, Color};

pub struct Button<M> {
	msg: Option<M>,
}

impl<M: Clone + 'static> Button<M> {
	pub fn new() -> Button<M> {
		Button { msg: None }
	}

	pub fn on_click(mut self, msg: M) -> Button<M> {
		self.msg = Some(msg);
		self
	}
}

impl<S: State> Widget<S> for Button<S::Message> {
	fn size(&mut self, _ctx: &mut SizeCtx<S>) -> Size {
		Size::new(100.0, 20.0)
	}

	fn event(&mut self, ctx: &mut EventCtx<S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				if let Some(msg) = &self.msg {
					ctx.send(msg.clone());
				}
			}
			_ => {}
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		let mut builder = lyon::path::Path::builder();
		builder.add_rounded_rectangle(
			&ctx.layout().rect,
			&BorderRadii::new(3.0),
			Winding::Positive,
		);
		let path = builder.build();

		ctx.fill(&path, Color([0.35, 0.3, 0.05, 1.0]));
	}
}
