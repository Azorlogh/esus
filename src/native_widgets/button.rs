use crate::{state::State, widget::prelude::*};

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
		ctx.rect(widget::paint::DrawMode::Fill, ctx.layout().rect);
		// ctx.stroke(rect(10, 10))
	}
}
