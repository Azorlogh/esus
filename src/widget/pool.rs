use crate::widget::{self, EventCtx, Id, Layout, LayoutCtx, Pod, SizeCtx, Widget};
use crate::{device::DeviceStates, event::Event};
use crate::{
	painter::Painter,
	render::{self, Renderer},
	widget::PaintCtx,
};
use crate::{Size, SizeConstraints};
use std::collections::{HashMap, VecDeque};

pub enum PoolMessage<S, M> {
	AddWidget {
		parent: Id,
		widget: Box<dyn Widget<S, M>>,
	},
}

pub struct Pool<S, M> {
	pub last_id: Id,
	widgets: HashMap<Id, widget::Pod<S, M>>,
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

	pub fn handle_message(&mut self, msg: PoolMessage<S, M>) {
		match msg {
			PoolMessage::AddWidget { parent, widget } => {
				self.add_child_widget(parent, widget);
			}
		}
	}

	pub fn set_root_widget(&mut self, widget: Box<dyn Widget<S, M>>) {
		let id = widget.id();
		let pod = Pod::new(None, widget);
		self.widgets.insert(id, pod);
	}

	pub fn add_child_widget(&mut self, parent_id: Id, widget: Box<dyn Widget<S, M>>) {
		let id = widget.id();
		if let Some(parent) = self.widgets.get_mut(&parent_id) {
			let pod = Pod::new(Some(parent_id), widget);
			parent.children.insert(id);
		} else {
			log::error!("specified parent does not exist");
		}
	}

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
		// figure out sizes
		{
			let ctx = SizeCtx {
				state: &mut state,
				sc: SizeConstraints {
					min: size,
					max: size,
				},
			};
		}
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
