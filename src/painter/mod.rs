use crate::render::{RenderCtx, Renderer};
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder};

mod rect;
use rect::Rect;

pub struct Painter {
	pub rect: Rect,
	pub glyph_brush: GlyphBrush<()>,
}

impl Painter {
	pub fn new(renderer: &Renderer) -> Painter {
		let rect = Rect::new(renderer);

		let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Ubuntu-M.ttf"))
			.expect("couldn't load font");
		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);

		Painter { rect, glyph_brush }
	}
}
