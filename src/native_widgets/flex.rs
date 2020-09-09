use crate::{
	widget::{self, Id, Layout, LayoutCtx, SizeCtx, ViewCtx, Widget},
	Axis, Rect, Size,
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

	fn layout(&mut self, ctx: &mut LayoutCtx<S>) -> Layout {
		let size = ctx.sc.max;
		// figure out non-flex children's total height
		let mut flex_total = 0.0;
		let mut remaining_height = size.height;
		for child in &mut self.children {
			flex_total += child.flex;
			if child.flex == 0.0 {
				let size = child.widget.layout(ctx, ctx.sc.loosen());
				remaining_height -= size.height;
			}
		}

		// lay the children out
		let mut curr_y = 0.0;
		for child in &mut self.children {
			if child.flex == 0.0 {
				let size = child.widget.layout(ctx, ctx.sc.loosen());
				child
					.widget
					.set_layout_rect(Rect::from_origin_size((0.0, curr_y), size));
				curr_y += size.height;
			} else {
				let size = Size::new(size.width, remaining_height * (child.flex / flex_total));
				child
					.widget
					.set_layout_rect(Rect::from_origin_size((0.0, curr_y), size));
				curr_y += size.height;
			}
		}

		log::warn!("flex layout done!");
		ctx.sc.max
	}
}
