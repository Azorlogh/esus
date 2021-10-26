use crate::painter::Painter;
use crate::render::RenderCtx;
use crate::{Layout, Rect};

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
	pub fn rect(&mut self, _: DrawMode, rect: Rect) {
		self.painter.rect.fill(
			&mut self.render_ctx,
			(rect.x0 as f32, rect.y0 as f32),
			(rect.x1 as f32, rect.y1 as f32),
		);
	}

	pub fn print(&mut self, rect: Rect, text: &str) {
		println!("drawing text with bounds {:?}", rect);
		let section = wgpu_glyph::Section::default()
			.add_text(
				wgpu_glyph::Text::new(text)
					.with_color([1.0, 1.0, 1.0, 1.0])
					.with_scale(12.0),
			)
			.with_screen_position((
				(rect.x0 as f32 + rect.x1 as f32) / 2.0,
				(rect.y0 as f32 + rect.y1 as f32) / 2.0,
			))
			.with_bounds((
				rect.x1 as f32 - rect.x0 as f32,
				rect.y1 as f32 - rect.y0 as f32,
			))
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

	pub fn layout(&self) -> Layout {
		self.layout.clone()
	}
}
