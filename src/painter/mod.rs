use crate::render::{RenderCtx, Renderer};

use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder};

mod brush;
use brush::Brush;
use winit::dpi::PhysicalSize;

pub struct Painter {
	pub brush: Brush,
	pub glyph_brush: GlyphBrush<()>,
}

impl Painter {
	pub fn new(renderer: &Renderer) -> Painter {
		println!("initializing brush");
		let brush = Brush::new(renderer);

		println!("initializing glyph brush");
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Ubuntu-M.ttf"))
			.expect("couldn't load font");
		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);

		Painter { brush, glyph_brush }
	}

	pub fn resize(&mut self, size: PhysicalSize<u32>) {
		self.brush.resize(size);
	}
}
