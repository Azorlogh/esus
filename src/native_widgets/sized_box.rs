use crate::{
	widget::{self, Id, LayoutCtx, SizeCtx, ViewCtx, Widget},
	Axis, Layout, Rect, Size, SizeConstraints,
};

pub struct SizedBox {
	id: Id,
	child: Option<Id>,
	width: Option<f64>,
	height: Option<f64>,
}

impl SizedBox {
	pub fn new<'a, S, M>(
		ctx: &mut ViewCtx<'a, S, M>,
		child: impl Widget<S, M> + 'static,
	) -> SizedBox {
		let id = ctx.acquire_id();
		SizedBox {
			id,
			child: Some(ctx.add_widget(id, child)),
			width: None,
			height: None,
		}
	}

	pub fn fix_width(mut self, width: f64) -> Self {
		self.width = Some(width);
		self
	}

	pub fn fix_height(mut self, height: f64) -> Self {
		self.height = Some(height);
		self
	}
}

impl<S, M> Widget<S, M> for SizedBox {
	fn id(&self) -> Id {
		self.id
	}

	fn size(&mut self, ctx: &mut SizeCtx<S, M>) -> Size {
		let mut size = if let Some(child) = self.child {
			ctx.get_size(child, ctx.sc)
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

	fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) -> Layout {
		if let Some(child) = self.child {
			ctx.set_layout(child, ctx.suggestion);
		}
		ctx.suggestion
	}
}
