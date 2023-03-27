use crate::{
	event::Event,
	state::State,
	widget::{self, HitCtx, LayoutCtx, PaintCtx, SizeCtx, Widget},
	Axis, Layout, Point, Rect, Size, SizeConstraints,
};

#[derive(Debug)]
pub struct ChildWidget<S> {
	widget: widget::Pod<S>,
	flex: f32,
}

#[derive(Debug)]
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

	pub fn with_child(self, c: impl Widget<S = S> + 'static) -> Flex<S> {
		self.with_flex_child(0.0, c)
	}

	pub fn with_flex_child(mut self, flex: f32, c: impl Widget<S = S> + 'static) -> Flex<S> {
		self.children.push(ChildWidget {
			widget: widget::Pod::new(c),
			flex,
		});
		self
	}
}

impl<S: State + std::fmt::Debug> Widget for Flex<S> {
	type S = S;

	fn size(&mut self, ctx: &mut SizeCtx<S>) -> Size {
		let mut minor_size = 0.0;
		for child in &mut self.children {
			if child.flex == 0.0 {
				let mut ctx = ctx.clone_with_size_constraints(SizeConstraints {
					min: Size::new(0.0, 0.0),
					max: ctx.sc.max,
				});
				let minor = self.axis.minor(child.widget.size(&mut ctx));
				minor_size = minor.max(minor_size);
			}
		}
		self.axis.with_minor(ctx.sc.max, minor_size)
	}

	fn layout(&mut self, mut ctx: LayoutCtx<S>) -> Layout {
		let suggestion = ctx.suggestion;
		let size = suggestion.rect.size;
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
				let mut ctx = ctx.create_size_context(constraints);
				let size = child.widget.size(&mut ctx);
				remaining_major -= self.axis.major(size);
			}
		}

		// lay the children out
		let mut curr_major = 0.0;
		for child in &mut self.children {
			let size = if child.flex == 0.0 {
				let mut ctx = ctx.create_size_context(constraints);
				child.widget.size(&mut ctx)
			} else {
				self.axis
					.with_major(size, remaining_major * (child.flex / flex_total))
			};
			let ctx = ctx.clone_with_layout(Layout {
				rect: Rect {
					origin: self.axis.with_major(Point::origin(), curr_major)
						+ suggestion.rect.origin.to_vector(),
					size,
				},
				depth: 0.0,
			});
			child.widget.layout(ctx);
			curr_major += self.axis.major(size);
		}

		suggestion
	}

	fn hit(&mut self, ctx: &HitCtx<S>) -> Option<f32> {
		self.children
			.iter_mut()
			.map(|child| {
				let hit = child.widget.hit(ctx);
				hit
			})
			.fold(None, |a, b| match (a, b) {
				(Some(a), Some(b)) => Some(a.max(b)),
				(Some(a), None) => Some(a),
				(None, Some(b)) => Some(b),
				(None, None) => None,
			})
	}

	fn event(&mut self, ctx: &mut widget::EventCtx<S>) {
		match ctx.event {
			Event::MouseDown(_) => {
				let hit = self.children.iter_mut().enumerate().fold(
					None,
					|acc: Option<(usize, f32)>, (idx, child)| {
						let mut hit_ctx = HitCtx::new(
							ctx.state,
							child.widget.layout.unwrap(),
							ctx.devices.mouse.pos,
						);
						let hit = child.widget.hit(&mut hit_ctx);

						match hit {
							None => acc,
							Some(depth) => match acc {
								None => Some((idx, depth)),
								Some((_, acc_depth)) => {
									if depth > acc_depth {
										Some((idx, depth))
									} else {
										acc
									}
								}
							},
						}
					},
				);
				match hit {
					None => {}
					Some((idx, _)) => {
						let child = &mut self.children[idx];
						child.widget.event(ctx);
					}
				}
			}
			_ => {
				for child in &mut self.children {
					child.widget.event(ctx);
				}
			}
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<S>) {
		for child in &mut self.children {
			child.widget.paint(ctx);
		}
	}
}
