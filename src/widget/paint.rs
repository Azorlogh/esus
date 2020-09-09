use crate::painter::Painter;
use crate::render::RenderCtx;
use crate::widget::Layout;
use kurbo::{Point, Rect};

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

	pub fn print(&mut self, pos: Point, text: &str) {
		let section = wgpu_glyph::Section::default()
			.add_text(wgpu_glyph::Text::new(text).with_color([1.0, 1.0, 1.0, 1.0]))
			.with_screen_position((pos.x as f32, pos.y as f32))
			.with_layout(wgpu_glyph::Layout::default().h_align(wgpu_glyph::HorizontalAlign::Left));

		self.painter.glyph_brush.queue(section);
		self.painter
			.glyph_brush
			.draw_queued(
				&self.render_ctx.device,
				&mut self.render_ctx.encoder,
				&self.render_ctx.frame.view,
				self.render_ctx.size.width,
				self.render_ctx.size.height,
			)
			.expect("something went wrong drawing glyphs");
	}

	pub fn layout(&self) -> Layout {
		self.layout.clone()
	}
}
