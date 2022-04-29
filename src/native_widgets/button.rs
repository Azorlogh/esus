use lyon::path::{builder::BorderRadii, traits::PathBuilder, Winding};

use crate::{
	state::State,
	widget::{prelude::*, HitCtx},
	Color,
};

#[derive(Debug)]
pub struct Button<S: State> {
	color: Color,
	msg: Option<S::Message>,
	child: Option<widget::Pod<S>>,
}

impl<S: State> Button<S> {
	pub fn new(child: impl Widget<S = S> + 'static) -> Self {
		let mut s = Self::empty();
		s.child = Some(widget::Pod::new(child));
		s
	}

	pub fn empty() -> Self {
		Self {
			child: None,
			color: Color([136.0 / 255.0, 192.0 / 255.0, 208.0 / 255.0, 1.0]),
			msg: None,
		}
	}

	pub fn with_color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}

	pub fn on_click(mut self, msg: S::Message) -> Self {
		self.msg = Some(msg);
		self
	}
}

impl<S: State> Widget for Button<S> {
	type S = S;

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		if let Some(child) = &mut self.child {
			child.size(ctx)
		} else {
			ctx.sc.max
		}
	}

	fn hit(&mut self, ctx: &HitCtx<S>) -> Option<f32> {
		Some(
			self.child
				.as_mut()
				.and_then(|child| child.hit(ctx))
				.unwrap_or(ctx.layout.depth),
		)
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<Self::S>) -> Layout {
		if let Some(child) = &mut self.child {
			child.layout(ctx);
		}
		ctx.suggestion
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

		ctx.fill(&path, self.color);

		if let Some(child) = &mut self.child {
			child.paint(ctx);
		}
	}
}
