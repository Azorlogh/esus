use std::fmt::Debug;

use crate::{
	state::{AnyMessage, AnyState},
	widget::{EventCtx, HitCtx, LayoutCtx, PaintCtx, SizeCtx, Widget},
	Layout, Size, State,
};

#[derive(Debug)]
pub struct DynamicState {
	pub state: Box<dyn AnyState>,
}

// #[derive(Clone)]
// pub struct DynamicMessage(Box<dyn AnyMessage>);

impl State for DynamicState {
	type Message = Box<dyn AnyMessage>;
}

// // trait AnyWidget: Debug {
// // 	fn hit(&mut self, _ctx: &HitCtx<DynamicState>) -> Option<f32>;
// // 	fn event(&mut self, _ctx: &mut EventCtx<DynamicState>);
// // 	fn size(&mut self, ctx: &mut SizeCtx<DynamicState>) -> Size;
// // 	fn layout(&mut self, ctx: LayoutCtx<DynamicState>) -> Layout;
// // 	fn paint(&mut self, _ctx: &mut PaintCtx<DynamicState>);
// // }

// #[derive(Debug)]
// pub struct DynamicWidget<S: State>(Box<dyn Widget<S = S>>);

// impl<S: State> Widget for DynamicWidget<S> {
// 	type S = DynamicState;

// 	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
// 		self.0.hit(ctx)
// 	}

// 	fn event(&mut self, _ctx: &mut EventCtx<Self::S>) {}

// 	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
// 		ctx.sc.max
// 	}

// 	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
// 		ctx.suggestion
// 	}

// 	fn paint(&mut self, _ctx: &mut PaintCtx<Self::S>) {}
// }
