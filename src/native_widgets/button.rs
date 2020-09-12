use crate::widget::prelude::*;

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

	pub fn register<'a, S>(self, ctx: &mut ViewCtx<'a, S, M>) -> Id {
		ctx.register(self)
	}
}

impl<S, M: Clone> Widget<S, M> for Button<M> {
	fn size(&mut self, _ctx: &mut SizeCtx<S, M>) -> Size {
		Size::new(100.0, 20.0)
	}

	fn event(&mut self, ctx: &mut EventCtx<S, M>) {
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
	}
}
