use lyon::path::Path;

use crate::painter::Painter;
use crate::render::RenderCtx;
use crate::{Color, Layout, Rect};

pub struct PaintCtx<'a, 'r, S> {
	pub render_ctx: &'a mut RenderCtx<'r>,
	pub painter: &'a mut Painter,
	pub state: &'a S,
	pub layout: Layout,
}

pub enum DrawMode {
	Fill,
	Stroke,
}

impl<'a, 'r, S> PaintCtx<'a, 'r, S> {
	pub fn fill(&mut self, path: &Path, color: Color) {
		self.painter.brush.set_color(self.render_ctx, color);
		self.painter.brush.fill(self.render_ctx, path);
	}

	pub fn print(&mut self, rect: Rect, text: &str, color: Color) {
		let section = wgpu_glyph::Section::default()
			.add_text(
				wgpu_glyph::Text::new(text)
					.with_color(color.0)
					.with_scale(12.0),
			)
			.with_screen_position(rect.center())
			.with_bounds(rect.size)
			.with_layout(
				wgpu_glyph::Layout::default()
					.h_align(wgpu_glyph::HorizontalAlign::Center)
					.v_align(wgpu_glyph::VerticalAlign::Center),
			);

		self.painter.glyph_brush.queue(section);
		self.painter
			.glyph_brush
			.draw_queued(
				&self.render_ctx.device,
				&mut self.render_ctx.staging_belt,
				&mut self.render_ctx.encoder,
				&self.render_ctx.view,
				self.render_ctx.size.width,
				self.render_ctx.size.height,
			)
			.expect("something went wrong drawing glyphs");
	}

	// pub fn fill()

	pub fn layout(&self) -> Layout {
		self.layout.clone()
	}
}
