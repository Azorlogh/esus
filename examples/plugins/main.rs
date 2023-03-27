use std::collections::VecDeque;

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;
use esus::{
	instance,
	native_widgets::{
		dynamic::DynamicState, Adapter, AdapterRef, Button, DropDown, Flex, Label, LabelText,
	},
	state::AnyMessage,
	widget::{self, EventCtx, HitCtx, LayoutCtx, PaintCtx, SizeCtx, Widget, WidgetExt},
	Color, Layout, Size,
};

#[derive(Debug)]
pub struct State {
	plugin_state: Box<dyn AnyPlugin>,
	plugin_id: u64,
}

#[derive(Clone, Debug)]
pub enum Message {
	Update,
	SetPlugin(Box<dyn AnyPlugin>),
	PluginMessage(Box<dyn AnyMessage>),
}

impl esus::State for State {
	type Message = Message;
}

fn main() {
	esus::init_simple_logger();

	let state = State {
		plugin_state: Box::new(FooPlugin(42)),
		plugin_id: 0,
	};
	let widget = state.plugin_state.get_gui();

	let mut instance = instance::Builder::<State>::new()
		.with_size((350.0, 500.0))
		.with_title("Epic test app")
		.with_state(state)
		.with_updater(|state, msg| match msg {
			Message::Update => {}
			Message::SetPlugin(plugin) => {
				state.plugin_state = plugin;
				state.plugin_id += 1;
			}
			Message::PluginMessage(msg) => {
				state.plugin_state.updater(msg);
			}
		})
		.with_view({
			Flex::column()
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(
							1.0,
							Button::new(Label::new("Foo"))
								.on_click(Message::SetPlugin(Box::new(FooPlugin::default()))),
						)
						.with_flex_child(
							1.0,
							Button::new(Label::new("Bar"))
								.on_click(Message::SetPlugin(Box::new(BarPlugin::default()))),
						),
				)
				.with_flex_child(
					1.0,
					PluginWidget {
						widget: widget::Pod::new(widget),
						last_id: 0,
					},
				)
		})
		.build();

	while !instance.is_dead() {
		instance.run_return();
	}
}

#[derive(Debug)]
pub struct PluginWidget {
	widget: widget::Pod<Box<dyn AnyPlugin>>,
	// widget: Box<dyn Widget<S = Box<dyn AnyPlugin>>>,
	last_id: u64,
}

impl Widget for PluginWidget {
	type S = State;

	fn size(&mut self, ctx: &mut SizeCtx<Self::S>) -> Size {
		let mut child_ctx = ctx.clone_with(&ctx.state.plugin_state, ctx.sc);
		self.widget.size(&mut child_ctx)
	}

	fn hit(&mut self, ctx: &HitCtx<Self::S>) -> Option<f32> {
		let child_ctx = HitCtx::new(&ctx.state.plugin_state, ctx.layout, ctx.point);
		self.widget.hit(&child_ctx)
	}

	fn layout(&mut self, mut ctx: LayoutCtx<Self::S>) -> Layout {
		let child_ctx = ctx.clone_with(&ctx.state.plugin_state, ctx.suggestion);
		self.widget.layout(child_ctx);
		self.widget.layout.unwrap()
	}

	fn event(&mut self, ctx: &mut EventCtx<Self::S>) {
		if self.last_id != ctx.state.plugin_id {
			self.widget = widget::Pod::new(ctx.state.plugin_state.get_gui());
			self.last_id = ctx.state.plugin_id;
			ctx.send(Message::Update);
		}

		let mut queue = VecDeque::new();

		let mut child_ctx = ctx.clone_with(&ctx.state.plugin_state, &mut queue);
		self.widget.event(&mut child_ctx);

		for msg in queue {
			ctx.send(Message::PluginMessage(msg));
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx<Self::S>) {
		let mut child_ctx = ctx.clone_with(&ctx.state.plugin_state, ctx.layout);
		self.widget.paint(&mut child_ctx);
	}
}

trait Plugin: esus::State {
	fn get_gui(&self) -> Box<dyn Widget<S = Self>>;

	fn updater(&mut self, msg: Self::Message);
}

pub trait AnyPlugin: DynClone + std::fmt::Debug + Downcast {
	fn get_gui(&self) -> Box<dyn Widget<S = Box<dyn AnyPlugin>>>;

	fn updater(&mut self, msg: Box<dyn AnyMessage>);
}
dyn_clone::clone_trait_object!(AnyPlugin);
impl_downcast!(AnyPlugin);

impl esus::State for Box<dyn AnyPlugin> {
	type Message = Box<dyn AnyMessage>;
}

impl<P: Plugin + Clone> AnyPlugin for P {
	fn get_gui(&self) -> Box<dyn Widget<S = Box<dyn AnyPlugin>>> {
		Box::new(AdapterRef::new(
			self.get_gui(),
			|s: &Box<dyn AnyPlugin>| s.downcast_ref::<P>().unwrap(),
			|msg| Box::new(msg),
		))
	}

	fn updater(&mut self, msg: Box<dyn AnyMessage>) {
		self.updater(*msg.downcast::<P::Message>().unwrap());
	}
}

#[derive(Debug, Clone, Default)]
struct FooPlugin(u64);

impl esus::State for FooPlugin {
	type Message = ();
}

impl Plugin for FooPlugin {
	fn get_gui(&self) -> Box<dyn Widget<S = Self>> {
		Box::new(
			Flex::column()
				.with_child(Label::new(LabelText::new_dynamic(|state: &FooPlugin| {
					format!("Hello from the Foo plugin :) {}", state.0)
				})))
				.with_child(Button::new(Label::new("+")).on_click(())),
		)
	}

	fn updater(&mut self, _: ()) {
		self.0 += 1;
	}
}

#[derive(Debug, Clone, Default)]
struct BarPlugin(String);

impl esus::State for BarPlugin {
	type Message = char;
}

impl Plugin for BarPlugin {
	fn get_gui(&self) -> Box<dyn Widget<S = Self>> {
		Box::new(
			Flex::column()
				.with_child(Label::new("hello from the Bar plugin :)"))
				.with_flex_child(
					1.0,
					Flex::row()
						.with_flex_child(1.0, Button::new(Label::new("a")).on_click('a'))
						.with_flex_child(1.0, Button::new(Label::new("b")).on_click('b'))
						.fix_height(100.0),
				)
				.with_child(Label::new(|state: &BarPlugin| format!("[ {} ]", state.0))),
		)
	}

	fn updater(&mut self, msg: char) {
		self.0.push(msg);
	}
}
