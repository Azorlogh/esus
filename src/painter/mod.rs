use crate::{
	render::{RenderCtx, Renderer},
	Rect, Size,
};

use wgpu::DepthStencilState;
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, GlyphCruncher};

mod brush;
use brush::Brush;
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct Painter {
	pub brush: Brush,
	pub glyph_brush: GlyphBrush<DepthStencilState>,
}

impl Painter {
	pub fn new(renderer: &Renderer) -> Painter {
		println!("initializing brush");
		let brush = Brush::new(renderer);

		println!("initializing glyph brush");
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Ubuntu-M.ttf"))
			.expect("couldn't load font");
		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.depth_stencil_state(wgpu::DepthStencilState {
				format: wgpu::TextureFormat::Depth32Float,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::LessEqual,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			})
			.build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);

		Painter { brush, glyph_brush }
	}

	pub fn resize(&mut self, size: PhysicalSize<u32>) {
		self.brush.resize(size);
	}

	pub fn measure_text(&self, rect: Rect, text: &str) -> Size {
		let section =
			wgpu_glyph::Section::default().add_text(wgpu_glyph::Text::new(text).with_scale(20.0));
		if let Some(bounds) = self.glyph_brush.glyph_bounds(section) {
			Size::new(bounds.width().ceil(), bounds.height().ceil())
		} else {
			Size::zero()
		}
	}
}
