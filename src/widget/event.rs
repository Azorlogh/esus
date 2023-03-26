use std::collections::VecDeque;

use crate::{device::DeviceStates, event::Event, state::State};

pub struct EventCtx<'a, S: State> {
	pub state: &'a S,
	pub event: &'a Event,
	pub devices: &'a DeviceStates,
	pub(crate) redraw_requested: &'a mut bool,
	pub(crate) msg_queue: &'a mut VecDeque<S::Message>,
}

impl<'a, S: State> EventCtx<'a, S> {
	pub fn new(
		event: &'a Event,
		state: &'a S,
		devices: &'a DeviceStates,
		redraw_requested: &'a mut bool,
		msg_queue: &'a mut VecDeque<S::Message>,
	) -> EventCtx<'a, S> {
		EventCtx {
			state,
			event,
			devices,
			redraw_requested,
			msg_queue,
		}
	}

	pub fn request_redraw(&mut self) {
		*self.redraw_requested = true;
	}

	pub fn send(&mut self, msg: S::Message) {
		self.msg_queue.push_back(msg);
	}
}
