use std::{collections::VecDeque, marker::PhantomData};

use crate::{
	state::State,
	widget::{prelude::*, HitCtx},
};

pub struct Adapter<SFrom: State, STo: State> {
	child: widget::Pod<SFrom>,
	from_state: Box<dyn for<'a> Fn(&'a STo) -> SFrom>,
	to_message: Box<dyn Fn(SFrom::Message) -> STo::Message>,
	_from: PhantomData<SFrom>,
	_to: PhantomData<STo>,
}

impl<SFrom: State, STo: State> std::fmt::Debug for Adapter<SFrom, STo> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Adapter")
			.field("child", &self.child)
			.finish()
	}
}

impl<SFrom: State, STo: State> Adapter<SFrom, STo> {
	pub fn new(
		child: impl Widget<S = SFrom> + 'static,
		from_state: impl Fn(&STo) -> SFrom + 'static,
		to_message: impl Fn(SFrom::Message) -> STo::Message + 'static,
	) -> Self {
		Self {
			child: widget::Pod::new(child),
			from_state: Box::new(from_state),
			to_message: Box::new(to_message),
			_from: PhantomData,
			_to: PhantomData,
		}
	}
}

impl<SFrom: State, STo: State> Widget for Adapter<SFrom, STo> {
	type S = STo;

	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = SizeCtx::new(&state, ctx.sc, ctx.painter);
		self.child.size(&mut child_ctx)
	}

	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
		let state = (self.from_state)(ctx.state);
		let child_ctx = HitCtx::new(&state, ctx.layout, ctx.point);
		self.child.hit(&child_ctx)
	}

	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
		let state = (self.from_state)(ctx.state);
		let child_ctx = LayoutCtx::new(&state, ctx.suggestion, ctx.painter);
		self.child.layout(child_ctx);
		self.child.layout.unwrap()
	}

	fn event(&mut self, ctx: &mut EventCtx<Self::S>) {
		let mut queue = VecDeque::new();
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = EventCtx::new(
			ctx.event,
			&state,
			ctx.devices,
			ctx.redraw_requested,
			&mut queue,
		);
		self.child.event(&mut child_ctx);

		for msg in queue {
			ctx.send((self.to_message)(msg));
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<Self::S>) {
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = PaintCtx {
			render_ctx: ctx.render_ctx,
			painter: ctx.painter,
			state: &state,
			layout: ctx.layout,
		};
		self.child.paint(&mut child_ctx);
	}
}

/////////////////////
//// Adapter Ref ////
/////////////////////

pub struct AdapterRef<SFrom: State, STo: State> {
	child: widget::Pod<SFrom>,
	from_state: Box<dyn for<'a> Fn(&'a STo) -> &SFrom>,
	to_message: Box<dyn Fn(SFrom::Message) -> STo::Message>,
	_from: PhantomData<SFrom>,
	_to: PhantomData<STo>,
}

impl<SFrom: State, STo: State> std::fmt::Debug for AdapterRef<SFrom, STo> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Adapter")
			.field("child", &self.child)
			.finish()
	}
}

impl<SFrom: State, STo: State> AdapterRef<SFrom, STo> {
	pub fn new(
		child: impl Widget<S = SFrom> + 'static,
		from_state: impl Fn(&STo) -> &SFrom + 'static,
		to_message: impl Fn(SFrom::Message) -> STo::Message + 'static,
	) -> Self {
		Self {
			child: widget::Pod::new(child),
			from_state: Box::new(from_state),
			to_message: Box::new(to_message),
			_from: PhantomData,
			_to: PhantomData,
		}
	}
}

impl<SFrom: State, STo: State> Widget for AdapterRef<SFrom, STo> {
	type S = STo;

	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = SizeCtx::new(state, ctx.sc, ctx.painter);
		self.child.size(&mut child_ctx)
	}

	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
		let state = (self.from_state)(ctx.state);
		let child_ctx = HitCtx::new(state, ctx.layout, ctx.point);
		self.child.hit(&child_ctx)
	}

	fn layout(&mut self, ctx: LayoutCtx<Self::S>) -> Layout {
		let state = (self.from_state)(ctx.state);
		let child_ctx = LayoutCtx::new(state, ctx.suggestion, ctx.painter);
		self.child.layout(child_ctx);
		self.child.layout.unwrap()
	}

	fn event(&mut self, ctx: &mut EventCtx<Self::S>) {
		let mut queue = VecDeque::new();
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = EventCtx::new(
			ctx.event,
			state,
			ctx.devices,
			ctx.redraw_requested,
			&mut queue,
		);
		self.child.event(&mut child_ctx);

		for msg in queue {
			ctx.send((self.to_message)(msg));
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<Self::S>) {
		let state = (self.from_state)(ctx.state);
		let mut child_ctx = PaintCtx {
			render_ctx: ctx.render_ctx,
			painter: ctx.painter,
			state: state,
			layout: ctx.layout,
		};
		self.child.paint(&mut child_ctx);
	}
}
