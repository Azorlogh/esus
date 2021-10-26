use crate::{
	state::State,
	widget::{self, LayoutCtx, PaintCtx, SizeCtx, Widget},
	Axis, Layout, Rect, Size, SizeConstraints,
};

#[derive(Debug)]
pub struct ChildWidget<S> {
	widget: widget::Pod<S>,
	flex: f64,
}

pub struct Flex<S> {
	axis: Axis,
	children: Vec<ChildWidget<S>>,
}

impl<S: State> Flex<S> {
	pub fn row() -> Flex<S> {
		Flex {
			axis: Axis::X,
			children: vec![],
		}
	}
	pub fn column() -> Flex<S> {
		Flex {
			axis: Axis::Y,
			children: vec![],
		}
	}

	pub fn with_child(self, c: impl Widget<S> + 'static) -> Flex<S> {
		self.with_flex_child(c, 0.0)
	}

	pub fn with_flex_child(mut self, c: impl Widget<S> + 'static, flex: f64) -> Flex<S> {
		self.children.push(ChildWidget {
			widget: widget::Pod::new(c),
			flex,
		});
		self
	}
}

impl<S: State + std::fmt::Debug> Widget<S> for Flex<S> {
	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		ctx.sc.max
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S>) -> Layout {
		let suggestion = ctx.suggestion;
		let size = suggestion.rect.size();
		let constraints = SizeConstraints {
			min: Size::new(0.0, 0.0),
			max: size,
		};
		// figure out non-flex children's total height
		let mut flex_total = 0.0;
		let mut remaining_major = self.axis.major(size);
		for child in &mut self.children {
			flex_total += child.flex;
			if child.flex == 0.0 {
				let mut ctx = SizeCtx::new(ctx.state, constraints);
				let size = child.widget.size(&mut ctx);
				remaining_major -= self.axis.major(size);
			}
		}

		// lay the children out
		let mut curr_major = 0.0;
		for child in &mut self.children {
			let size = if child.flex == 0.0 {
				let mut ctx = SizeCtx::new(ctx.state, constraints);
				child.widget.size(&mut ctx)
			} else {
				Size::new(size.width, remaining_major * (child.flex / flex_total))
			};
			let mut ctx = LayoutCtx::new(
				ctx.state,
				Layout {
					rect: Rect::from_origin_size(
						self.axis.with_major((0.0, 0.0), curr_major),
						size,
					),
					depth: 0.0,
				},
			);
			child.widget.layout(&mut ctx);
			curr_major += self.axis.major(size);
		}

		println!("{:?}", self.children);

		log::warn!("flex layout done!");
		suggestion
	}

	fn event(&mut self, ctx: &mut widget::EventCtx<S>) {
		for child in &mut self.children {
			child.widget.event(ctx);
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		for child in &mut self.children {
			child.widget.paint(ctx);
		}
	}
}
