use std::{collections::VecDeque, marker::PhantomData};

use crate::{
	state::State,
	widget::{prelude::*, HitCtx},
};

pub struct Adapter<SParent: State, SChild: State, AdaptState>
where
	AdaptState: for<'a> AdapterFunc<'a, SParent>,
{
	child: widget::Pod<SChild>,
	// from_state: Box<dyn for<'a> Fn(&'a STo) -> SFrom>,
	from_state: AdaptState,
	_from: PhantomData<SParent>,
	// to_message: Box<dyn Fn(SFrom::Message) -> STo::Message>,
	// _to: PhantomData<SChild>,
}

pub trait AdapterFunc<'a, SParent> {
	type Output: 'a;
	fn call(&self, foo: &'a SParent) -> Self::Output;
}

impl<'a, 'p: 'a, F, SParent: 'p, Ret: 'a> AdapterFunc<'a, SParent> for F
where
	F: Fn(&'a SParent) -> Ret,
{
	type Output = Ret;
	fn call(&self, foo: &'a SParent) -> Self::Output {
		(self)(foo)
	}
}

impl<SChild: State, SParent: State, AdaptState> std::fmt::Debug
	for Adapter<SParent, SChild, AdaptState>
where
	AdaptState: for<'a> AdapterFunc<'a, SParent>,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Adapter")
			.field("child", &self.child)
			.finish()
	}
}

impl<SChild: State, SParent: State, AdaptState> Adapter<SParent, SChild, AdaptState>
where
	AdaptState: for<'a> AdapterFunc<'a, SParent, Output = SChild>,
{
	pub fn new(
		child: impl Widget<S = SChild> + 'static,
		from_state: AdaptState,
		// to_message: impl Fn(SFrom::Message) -> STo::Message + 'static,
	) -> Self {
		Self {
			child: widget::Pod::new(child),
			from_state,
			_from: PhantomData,
			// to_message: Box::new(to_message),
			// _to: PhantomData,
		}
	}
}

impl<SChild: State, SParent: State, AdaptState> Widget for Adapter<SParent, SChild, AdaptState>
where
	AdaptState: for<'a> AdapterFunc<'a, SParent, Output = SChild>,
{
	type S = SParent;

	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		let state = self.from_state.call(ctx.state);
		let mut child_ctx = SizeCtx::new(&state, ctx.sc, ctx.painter);
		self.child.size(&mut child_ctx)
	}

	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
		let state = self.from_state.call(ctx.state);
		let child_ctx = HitCtx::new(&state, ctx.layout, ctx.point);
		self.child.hit(&child_ctx)
	}

	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
		let state = self.from_state.call(ctx.state);
		let child_ctx = LayoutCtx::new(&state, ctx.suggestion, ctx.painter);
		self.child.layout(child_ctx);
		self.child.layout.unwrap()
	}

	fn event(&mut self, ctx: &mut EventCtx<Self::S>) {
		let mut queue = VecDeque::new();
		let state = self.from_state.call(ctx.state);
		let mut child_ctx = EventCtx::new(
			ctx.event,
			&state,
			ctx.devices,
			ctx.redraw_requested,
			&mut queue,
		);
		self.child.event(&mut child_ctx);

		for msg in queue {
			// ctx.send((self.to_message)(msg));
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<Self::S>) {
		let state = self.from_state.call(ctx.state);
		let mut child_ctx = PaintCtx {
			render_ctx: ctx.render_ctx,
			painter: ctx.painter,
			state: &state,
			layout: ctx.layout,
		};
		self.child.paint(&mut child_ctx);
	}
}
