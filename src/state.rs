use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;

pub trait State: std::fmt::Debug + 'static {
	type Message: Message;
}

pub trait Message: std::fmt::Debug + Clone {}
// dyn_clone::clone_trait_object!(Message);
impl<T: std::fmt::Debug + Clone> Message for T {}

pub trait AnyState: std::fmt::Debug + Downcast {
	// type Message;
}
impl_downcast!(AnyState);

impl<S: State + 'static> AnyState for S {}

pub trait AnyMessage: std::fmt::Debug + DynClone + Downcast {}
dyn_clone::clone_trait_object!(AnyMessage);
impl_downcast!(AnyMessage);

impl<T: Message + 'static> AnyMessage for T {}
