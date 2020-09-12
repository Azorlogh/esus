use crate::{
	widget::{Id, LayoutCtx, SizeCtx, ViewCtx, Widget},
	Layout, Size,
};

pub struct SizedBox {
	child: Option<Id>,
	width: Option<f64>,
	height: Option<f64>,
}

impl SizedBox {
	pub fn new(child: Id) -> SizedBox {
		let id = SizedBox {
			child: Some(child),
			width: None,
			height: None,
		};
		id
	}

	pub fn register<'a, S, M>(self, ctx: &mut ViewCtx<'a, S, M>) -> Id {
		let child = self.child;
		let id = ctx.register(self);
		if let Some(child) = child {
			ctx.set_child(id, child);
		}
		id
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
