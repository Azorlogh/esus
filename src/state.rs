use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;

pub trait State: std::fmt::Debug {
	type Message: Message;
}

pub trait Message: std::fmt::Debug + Clone {}
// dyn_clone::clone_trait_object!(Message);
impl<T: std::fmt::Debug + Clone> Message for T {}

pub trait AnyState: std::fmt::Debug + Downcast {
	// type Message;
}
impl_downcast!(AnyState);

pub trait AnyMessage: std::fmt::Debug + DynClone {}
dyn_clone::clone_trait_object!(AnyMessage);

impl<T: Message> AnyMessage for T {}

// impl<T: State> AnyState for T {
// 	// type Message = Box<dyn AnyMessage>;
// }
