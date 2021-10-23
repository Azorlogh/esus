use crate::{device::DeviceStates, event::Event, state::State};
use std::collections::VecDeque;

pub struct EventCtx<'a, S: State> {
	pub state: &'a S,
	pub event: &'a Event,
	pub devices: &'a DeviceStates,
	msg_queue: &'a mut VecDeque<S::Message>,
}

impl<'a, S: State> EventCtx<'a, S> {
	pub fn new(
		event: &'a Event,
		state: &'a S,
		devices: &'a DeviceStates,
		msg_queue: &'a mut VecDeque<S::Message>,
	) -> EventCtx<'a, S> {
		EventCtx {
			state,
			event,
			devices,
			msg_queue,
		}
	}

	pub fn send(&mut self, msg: S::Message) {
		self.msg_queue.push_back(msg);
	}
}
