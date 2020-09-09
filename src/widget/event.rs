use crate::{device::DeviceStates, event::Event};
use std::collections::VecDeque;

pub struct EventCtx<'a, S, M> {
	pub state: &'a S,
	pub event: &'a Event,
	pub devices: &'a DeviceStates,
	msg_queue: &'a mut VecDeque<M>,
}

impl<'a, S, M> EventCtx<'a, S, M> {
	pub fn new(
		event: &'a Event,
		state: &'a S,
		devices: &'a DeviceStates,
		msg_queue: &'a mut VecDeque<M>,
	) -> EventCtx<'a, S, M> {
		EventCtx {
			state,
			event,
			devices,
			msg_queue,
		}
	}

	pub fn send(&mut self, msg: M) {
		self.msg_queue.push_back(msg);
	}
}
