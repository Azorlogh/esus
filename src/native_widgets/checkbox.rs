use lyon::path::{builder::BorderRadii, traits::PathBuilder, Winding};

use crate::{
	state::State,
	widget::{prelude::*, HitCtx},
	Color,
};

type CheckboxState = bool;
#[derive(Debug, Clone)]
pub struct CheckboxMessage(pub bool);
impl State for CheckboxState {
	type Message = CheckboxMessage;
}

#[derive(Debug)]
pub struct Checkbox;

impl Checkbox {
	pub fn new() -> Self {
		Self
	}
}

impl Widget for Checkbox {
	type S = CheckboxState;

	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		ctx.sc.max
	}

	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
		Some(ctx.layout.depth)
	}

	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
		ctx.suggestion
	}

	fn event(&mut self, ctx: &mut EventCtx<Self::S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				ctx.send(CheckboxMessage(!ctx.state));
			}
			_ => {}
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<Self::S>) {
		let mut builder = lyon::path::Path::builder();
		builder.add_rounded_rectangle(
			&ctx.layout().rect,
			&BorderRadii::new(3.0),
			Winding::Positive,
		);
		let path = builder.build();

		if *ctx.state {
			ctx.fill(&path, Color([0.0, 1.0, 0.0, 1.0]));
		} else {
			ctx.fill(&path, Color([1.0, 0.0, 0.0, 1.0]));
		}
	}
}
