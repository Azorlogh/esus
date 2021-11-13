use crate::{
	state::State,
	widget::{self, LayoutCtx, SizeCtx, Widget},
	Layout, Size,
};

pub struct SizedBox<S: State> {
	child: Option<widget::Pod<S>>,
	width: Option<f32>,
	height: Option<f32>,
}

impl<S: State> SizedBox<S> {
	pub fn new(child: impl Widget<S> + 'static) -> SizedBox<S> {
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

impl<S: State> Widget<S> for SizedBox<S> {
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
}
