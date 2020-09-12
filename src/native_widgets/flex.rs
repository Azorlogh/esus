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
	children: Vec<ChildWidget>,
}

impl Flex {
	pub fn row() -> Flex {
		Flex {
			axis: Axis::X,
			children: vec![],
		}
	}
	pub fn column() -> Flex {
		Flex {
			axis: Axis::Y,
			children: vec![],
		}
	}

	pub fn with_child(mut self, c: widget::Id) -> Flex {
		self.children.push(ChildWidget { id: c, flex: 0.0 });
		self
	}

	pub fn with_flex_child(mut self, c: widget::Id, flex: f64) -> Flex {
		self.children.push(ChildWidget { id: c, flex });
		self
	}

	pub fn register<'a, S, M>(self, ctx: &mut ViewCtx<'a, S, M>) -> Id {
		let children = self.children.iter().map(|c| c.id).collect::<Vec<Id>>();
		let id = ctx.register(self);
		for child in children {
			ctx.set_child(id, child);
		}
		id
	}
}

impl<S, M> Widget<S, M> for Flex {
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
		let mut remaining_major = self.axis.major(size);
		for child in &mut self.children {
			flex_total += child.flex;
			if child.flex == 0.0 {
				let size = ctx.get_size(child.id, constraints);
				remaining_major -= self.axis.major(size);
			}
		}

		// lay the children out
		let mut curr_major = 0.0;
		for child in &mut self.children {
			let size = if child.flex == 0.0 {
				ctx.get_size(child.id, constraints)
			} else {
				Size::new(size.width, remaining_major * (child.flex / flex_total))
			};
			ctx.set_layout(
				child.id,
				Layout {
					rect: Rect::from_origin_size(
						self.axis.with_major((0.0, 0.0), curr_major),
						size,
					),
					depth: 0.0,
				},
			);
			curr_major += self.axis.major(size);
		}

		log::warn!("flex layout done!");
		suggestion
	}
}
