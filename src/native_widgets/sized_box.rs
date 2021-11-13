use crate::{
	state::State,
	widget::{self, EventCtx, LayoutCtx, PaintCtx, SizeCtx, Widget},
	Layout, Size,
};

#[derive(Debug)]
pub struct SizedBox<S: State> {
	child: Option<widget::Pod<S>>,
	width: Option<f32>,
	height: Option<f32>,
}

impl<S: State> SizedBox<S> {
	pub fn new(child: impl Widget<S = S> + 'static) -> SizedBox<S> {
		let id = SizedBox {
			child: Some(widget::Pod::new(child)),
			width: None,
			height: None,
		};
		id
	}

	pub fn fix_width(mut self, width: f32) -> Self {
		self.width = Some(width);
		self
	}

	pub fn fix_height(mut self, height: f32) -> Self {
		self.height = Some(height);
		self
	}
}

impl<S: State> Widget for SizedBox<S> {
	type S = S;

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		let mut size = if let Some(child) = &mut self.child {
			child.size(ctx)
		} else {
			ctx.sc.max
		};
		if let Some(width) = self.width {
			size.width = width;
		}
		if let Some(height) = self.height {
			size.height = height;
		}
		size
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S>) -> Layout {
		if let Some(child) = &mut self.child {
			child.layout(ctx);
		}
		ctx.suggestion
	}

	fn event(&mut self, ctx: &mut EventCtx<S>) {
		if let Some(child) = &mut self.child {
			child.event(ctx);
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		if let Some(child) = &mut self.child {
			child.paint(ctx);
		}
	}
}
