use crate::{
	widget::{self, Id, LayoutCtx, SizeCtx, ViewCtx, Widget},
	Axis, Layout, Rect, Size, SizeConstraints,
};

pub struct ChildWidget {
	id: widget::Id,
	flex: f64,
}

pub struct Flex {
	axis: Axis,
	id: Id,
	children: Vec<ChildWidget>,
}

impl Flex {
	pub fn row<'a, S, M>(ctx: &mut ViewCtx<'a, S, M>) -> Flex {
		Flex {
			axis: Axis::X,
			id: ctx.acquire_id(),
			children: vec![],
		}
	}
	pub fn column<'a, S, M>(ctx: &mut ViewCtx<'a, S, M>) -> Flex {
		Flex {
			axis: Axis::Y,
			id: ctx.acquire_id(),
			children: vec![],
		}
	}

	pub fn with_child<'a, S, M>(
		mut self,
		ctx: &mut ViewCtx<'a, S, M>,
		w: impl Widget<S, M> + 'static,
	) -> Flex {
		self.children.push(ChildWidget {
			id: ctx.add_widget(self.id, w),
			flex: 0.0,
		});
		self
	}

	pub fn with_flex_child<'a, S, M>(
		mut self,
		ctx: &mut ViewCtx<'a, S, M>,
		w: impl Widget<S, M> + 'static,
		flex: f64,
	) -> Flex {
		self.children.push(ChildWidget {
			id: ctx.add_widget(self.id, w),
			flex,
		});
		self
	}
}

impl<S, M> Widget<S, M> for Flex {
	fn id(&self) -> Id {
		self.id
	}

	// fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
	// 	let mut width: f64 = ctx.sc.width;

	// 	let mut height: f64 = 0.0;
	// 	for id in &self.children {
	// 		let size = ctx.sizes[id];
	// 		let rect = Rect::new(0.0, height, width, height + size.height);
	// 		ctx.layout_suggestions
	// 			.insert(*id, Layout { rect, depth: 0.0 });
	// 		height += size.height;
	// 	}
	// 	Size { width, height }
	// }

	fn size(&mut self, ctx: &mut SizeCtx<S, M>) -> Size {
		ctx.sc.max
	}

	fn layout(&mut self, ctx: &mut LayoutCtx<S, M>) -> Layout {
		let suggestion = ctx.suggestion;
		let size = suggestion.rect.size();
		let constraints = SizeConstraints {
			min: Size::new(0.0, 0.0),
			max: size,
		};
		// figure out non-flex children's total height
		let mut flex_total = 0.0;
		let mut remaining_height = size.height;
		for child in &mut self.children {
			flex_total += child.flex;
			if child.flex == 0.0 {
				let size = ctx.get_size(child.id, constraints);
				remaining_height -= size.height;
			}
		}

		// lay the children out
		let mut curr_y = 0.0;
		for child in &mut self.children {
			if child.flex == 0.0 {
				let size = ctx.get_size(child.id, constraints);
				ctx.set_layout(
					child.id,
					Layout {
						rect: Rect::from_origin_size((0.0, curr_y), size),
						depth: 0.0,
					},
				);
				curr_y += size.height;
			} else {
				let size = Size::new(size.width, remaining_height * (child.flex / flex_total));
				ctx.set_layout(
					child.id,
					Layout {
						rect: Rect::from_origin_size((0.0, curr_y), size),
						depth: 0.0,
					},
				);
				curr_y += size.height;
			}
		}

		log::warn!("flex layout done!");
		suggestion
	}
}
