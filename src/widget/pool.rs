use crate::widget::{self, EventCtx, Id, Layout, LayoutCtx, Pod, Widget};
use crate::{device::DeviceStates, event::Event};
use crate::{
	painter::Painter,
	render::{self, Renderer},
	widget::PaintCtx,
};
use crate::{Rect, Size};
use pl_lens::Lens;
use std::collections::{HashMap, VecDeque};

pub enum PoolMessage<S, M> {
	AddWidget {
		parent: Id,
		widget: Box<dyn Widget<S, M>>,
	},
}

pub struct Pool<S, M> {
	pub last_id: Id,
	pub widgets: HashMap<Id, widget::Pod<S, Box<dyn std::any::Any>, M>>,
	root: Option<Id>,
	order: Vec<Id>,
}

impl<S, M> Pool<S, M> {
	pub fn new() -> Pool<S, M> {
		Pool {
			last_id: Id::initial(),
			widgets: HashMap::new(),
			root: None,
			order: vec![],
		}
	}

	// pub fn handle_message(&mut self, msg: PoolMessage<S, M>) {
	// 	match msg {
	// 		PoolMessage::AddWidget { parent, widget } => {
	// 			self.add_child_widget(parent, widget);
	// 		}
	// 	}
	// }

	pub fn set_root_widget(&mut self, id: Id) {
		self.root = Some(id);
	}

	pub fn add_widget<D>(
		&mut self,
		widget: impl Widget<Box<D>, M> + 'static,
		lens: impl Lens<Source = S, Target = D> + 'static,
	) -> Id {
		let id = self.last_id.next();
		let pod = Pod::new(None, Box::new(widget), lens);
		self.widgets.insert(id, pod);
		id
	}

	pub fn set_widget_child(&mut self, parent_id: Id, child_id: Id) {
		let mut parent = self
			.widgets
			.remove(&parent_id)
			.expect("specified parent does not exist");
		let mut child = self
			.widgets
			.remove(&child_id)
			.expect("specified child does not exist");
		parent.children.insert(child_id);
		child.parent = Some(parent_id);
		self.widgets.insert(parent_id, parent);
		self.widgets.insert(child_id, child);
	}

	// pub fn add_child_widget(&mut self, parent_id: Id, widget: Box<dyn Widget<S, M>>) {
	// 	let id = widget.id();
	// 	println!("adding child widget {:?} to {:?}", id, parent_id);
	// 	let mut parent = self
	// 		.widgets
	// 		.remove(&parent_id)
	// 		.expect("specified parent does not exist");
	// 	let pod = Pod::new(Some(parent_id), widget);
	// 	self.widgets.insert(id, pod);
	// 	parent.children.insert(id);
	// 	self.widgets.insert(parent_id, parent);
	// }

	// pub fn resolve_layout<'a>(&mut self, state: &'a S) {
	// 	let layouts = vec![];

	// 	// acquire layouts
	// 	for (id, widget) in &mut self.widgets {
	// 		let mut ctx = LayoutCtx::new(state, layout_suggestions[id].clone());
	// 		let layout = widget.layout(&mut ctx);
	// 		{
	// 			let mut i = 0;
	// 			while i < self.order.len() {
	// 				let other_id = self.order[i];
	// 				let other_depth = depths[&other_id];
	// 				if layout.depth <= other_depth {
	// 					break;
	// 				}
	// 				i += 1;
	// 			}
	// 			self.order.insert(i, *id);
	// 			depths.insert(*id, layout.depth);
	// 		}
	// 		widget.layout = Some(layout);
	// 	}
	// }

	pub fn resolve_layout<'a>(&mut self, state: &'a S, size: Size) {
		let root_id = self.root.unwrap();
		let mut root = self.widgets.remove(&root_id).unwrap(); // fix this
		let mut ctx = LayoutCtx::new(
			state,
			self,
			Layout {
				rect: Rect::from_origin_size((0.0, 0.0), size),
				depth: 0.0,
			},
		);
		root.layout(&mut ctx);

		let mut order: Vec<Id> = self
			.widgets
			.iter()
			.filter_map(|(&id, widget)| {
				if widget.layout.is_some() {
					Some(id)
				} else {
					None
				}
			})
			.collect();
		order.sort_by(|a, b| {
			let a_depth = self.widgets.get(a).unwrap().layout.unwrap().depth;
			let b_depth = self.widgets.get(b).unwrap().layout.unwrap().depth;
			a_depth.partial_cmp(&b_depth).unwrap()
		});
		self.order = order;
	}

	pub fn paint(&mut self, renderer: &mut Renderer, painter: &mut Painter, state: &S) {
		let mut render_ctx =
			render::next_frame(&mut renderer.device, &mut renderer.swapchain, renderer.size);

		for id in self.order.iter().rev() {
			let widget = self.widgets.get_mut(id).unwrap();
			let mut paint_ctx = PaintCtx {
				render_ctx: &mut render_ctx,
				painter,
				state,
				layout: widget.layout.clone().unwrap(),
			};
			widget.paint(&mut paint_ctx);
		}
		render::finish_frame(&mut renderer.queue, render_ctx);
	}

	#[allow(unreachable_patterns)]
	pub fn event(
		&mut self,
		evt: Event,
		state: &S,
		devices: &DeviceStates,
		messages: &mut VecDeque<M>,
	) {
		if let Some(mut evt) = match evt {
			Event::MouseDown(_) | Event::MouseMove(_) | Event::MouseUp(_) => Some(evt),
			_ => None,
		} {
			for id in self.order.iter() {
				let widget = self.widgets.get_mut(id).unwrap();
				let layout = widget.layout.clone().unwrap();

				if layout.rect.contains(devices.mouse.pos) {
					if let Event::MouseMove(evt) = &mut evt {
						evt.old_pos = (evt.screen_old_pos - layout.rect.origin()).to_point();
						evt.pos = (evt.screen_pos - layout.rect.origin()).to_point();
					}

					let mut ctx = EventCtx::new(&evt, state, devices, messages);

					widget.event(&mut ctx);
				}
			}
		}
	}
}
