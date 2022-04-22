use crate::SizeConstraints;

#[derive(Debug)]
pub struct SizeCtx<'a, S> {
	pub state: &'a S,
	pub sc: SizeConstraints,
}

impl<'a, S> SizeCtx<'a, S> {
	pub fn new(state: &'a S, sc: SizeConstraints) -> SizeCtx<'a, S> {
		SizeCtx { state, sc }
	}

	// pub fn get_size(&mut self, id: widget::Id, sc: SizeConstraints) -> Size {
	// 	let mut child = self
	// 		.pool
	// 		.widgets
	// 		.remove(&id)
	// 		.expect("tried to get size of non-existent widget");
	// 	let mut ctx = SizeCtx {
	// 		state: self.state,
	// 		sc,
	// 	};
	// 	let size = child.size(&mut ctx);
	// 	self.pool.widgets.insert(id, child);
	// 	size
	// }
}
