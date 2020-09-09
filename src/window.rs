use crate::widget::Widget;
use winit::dpi::PhysicalSize;

pub struct WindowDesc<D> {
	pub title: String,
	pub size: winit::dpi::PhysicalSize<u32>,
	pub root: Box<dyn Widget<D>>,
}

impl<D> WindowDesc<D> {
	pub fn new<W>(builder: impl Fn() -> W + 'static) -> WindowDesc<D>
	where
		W: Widget<D> + 'static,
	{
		WindowDesc {
			title: String::from("Default App Name"),
			size: (0, 0).into(),
			root: Box::new(builder()),
		}
	}

	pub fn with_title(mut self, title: &str) -> Self {
		self.title = title.to_string();
		self
	}

	pub fn with_size(mut self, size: PhysicalSize<u32>) -> Self {
		self.size = size;
		self
	}
}
