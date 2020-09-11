trait WidgetExt {
	fn fix_width(self, width: f64) -> SizedBox<S> {
		SizedBox::new(self).width(width)
	}

	fn fix_height(self, height: f64) -> SizedBox<S> {
		SizedBox::new(self).height(height)
	}
}

impl<S, M, W: Widget<S, M>> WidgetExt<S, M> for W {}
