use crate::render::{RenderCtx, Renderer};

use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder};

mod rect;
use rect::Rect;

mod brush;
use brush::Brush;

pub struct Painter {
	pub rect: Rect,
	pub brush: Brush,
	pub glyph_brush: GlyphBrush<()>,
}

impl Painter {
	pub fn new(renderer: &Renderer) -> Painter {
		println!("initializing rect painter");
		let rect = Rect::new(renderer);

		println!("initializing brush");
		let brush = Brush::new(renderer);

		println!("initializing glyph brush");
		let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Ubuntu-M.ttf"))
			.expect("couldn't load font");
		let glyph_brush = GlyphBrushBuilder::using_font(font)
			.build(&renderer.device, wgpu::TextureFormat::Bgra8UnormSrgb);

		Painter {
			rect,
			brush,
			glyph_brush,
		}
	}
}
