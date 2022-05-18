use lyon::path::{traits::PathBuilder, Winding};

use crate::{
	state::State,
	widget::{self, prelude::*, EventCtx, LayoutCtx, PaintCtx, SizeCtx, Widget},
};

#[derive(Debug)]
pub struct SizedBox<S: State> {
	child: Option<widget::Pod<S>>,
	width: Option<f32>,
	height: Option<f32>,
	padding: Option<f32>,
	background: Option<Color>,
	align: Align2,
}

impl<S: State> SizedBox<S> {
	pub fn new(child: impl Widget<S = S> + 'static) -> SizedBox<S> {
		let id = SizedBox {
			child: Some(widget::Pod::new(child)),
			width: None,
			height: None,
			padding: None,
			background: None,
			align: Align2::default(),
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

	pub fn with_padding(mut self, padding: f32) -> Self {
		self.padding = Some(padding);
		self
	}

	pub fn with_background(mut self, background: Color) -> Self {
		self.background = Some(background);
		self
	}

	pub fn align_horizontal(mut self, align: Align) -> Self {
		self.align.x = align;
		self
	}

	pub fn align_vertical(mut self, align: Align) -> Self {
		self.align.y = align;
		self
	}

	pub fn align(mut self, align: Align2) -> Self {
		self.align = align;
		self
	}
}

impl<S: State> Widget for SizedBox<S> {
	type S = S;

	fn hit(&mut self, _ctx: &widget::HitCtx<S>) -> Option<f32> {
		None
	}

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		if let Some(width) = self.width {
			ctx.sc.max.width = ctx.sc.max.width.min(width);
		}
		if let Some(height) = self.height {
			ctx.sc.max.height = ctx.sc.max.height.min(height);
		}
		if let Some(padding) = self.padding {
			ctx.sc.max -= Size::new(padding * 2.0, padding * 2.0);
		}
		let size = if let Some(child) = &mut self.child {
			child.size(ctx)
		} else {
			ctx.sc.max
		};
		size
	}

	fn layout(&mut self, mut ctx: LayoutCtx<S>) -> Layout {
		let mut origin = ctx.suggestion.rect.origin;
		let suggestion_size = ctx.suggestion.rect.size;
		let target_size = Size::new(
			self.width.unwrap_or(suggestion_size.width) - self.padding.unwrap_or(0.0) * 2.0,
			self.height.unwrap_or(suggestion_size.height) - self.padding.unwrap_or(0.0) * 2.0,
		);

		match self.align.x {
			Align::Min => {}
			Align::Center => {
				origin.x = origin.x + (suggestion_size.width - target_size.width) / 2.0;
			}
			Align::Max => {
				origin.x = origin.x + suggestion_size.width - target_size.width;
			}
		}

		match self.align.y {
			Align::Min => {}
			Align::Center => {
				origin.y = origin.y + (suggestion_size.height - target_size.height) / 2.0;
			}
			Align::Max => {
				origin.y = origin.y + suggestion_size.height - target_size.height;
			}
		}

		let layout = Layout {
			rect: Rect {
				origin,
				size: target_size,
			},
			depth: ctx.suggestion.depth,
		};

		if let Some(child) = &mut self.child {
			child.layout(ctx.clone_with_layout(layout));
		}
		ctx.suggestion
	}

	fn event(&mut self, ctx: &mut EventCtx<S>) {
		if let Some(child) = &mut self.child {
			child.event(ctx);
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		if let Some(background) = self.background {
			let mut builder = lyon::path::Path::builder();
			builder.add_rectangle(&ctx.layout().rect, Winding::Positive);
			let path = builder.build();
			ctx.fill(&path, background);
		}
		if let Some(child) = &mut self.child {
			child.paint(ctx);
		}
	}
}
