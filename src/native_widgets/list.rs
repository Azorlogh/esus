use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::widget::prelude::*;
use crate::State;

// trait ListIter<T> {
// 	fn next(&mut self) -> Option<T>;
// }

// impl<T> ListIter for T
// where
// 	T: Iterator<Item = &mut Widget>,
// {
// 	fn next(&mut self) -> Option<&mut Widget> {
// 		self.next()
// 	}
// }

/// Message

#[derive(Debug, Clone)]
pub struct ListMessage<M> {
	pub idx: usize,
	pub message: M,
}

/// State

#[derive(Debug)]
pub struct ListState<T, I>(pub T, pub PhantomData<I>)
where
	T: Debug,
	T: Iterator,
	T::Item: Borrow<I>,
	I: State;

impl<T, I> State for ListState<T, I>
where
	T: Debug,
	T: Iterator,
	T::Item: Borrow<I>,
	I: State,
	I::Message: std::fmt::Debug,
{
	type Message = ListMessage<I::Message>;
}

pub struct List<S, I: State> {
	create_widget: Box<dyn Fn() -> Box<dyn Widget<S = I>>>,
	// create_message: Box<dyn Fn(usize, I::Message) -> S::Message>,
	children: Vec<widget::Pod<I>>,
	phantom: PhantomData<S>,
}

impl<S, I: State> std::fmt::Debug for List<S, I> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("List").finish()
	}
}

impl<S, I: State> List<S, I> {
	pub fn new<W: Widget<S = I> + 'static>(
		create_widget: impl Fn() -> W + 'static,
		// create_message: impl Fn(usize, I::Message) -> S::Message + 'static,
	) -> Self {
		Self {
			create_widget: Box::new(move || Box::new(create_widget())),
			children: Vec::new(),
			phantom: PhantomData,
		}
	}
}

impl<S, I> Widget for List<S, I>
where
	// S: AsRef<dyn Iterator<Item = I>>,
	S: Debug,
	S: Iterator,
	S::Item: Borrow<I>,
	I: State,
{
	type S = ListState<S, I>;

	fn hit(&mut self, _ctx: &widget::HitCtx<Self::S>) -> Option<f32> {
		None
	}

	fn event(&mut self, _ctx: &mut widget::EventCtx<Self::S>) {}

	fn size(&mut self, ctx: &mut widget::SizeCtx<Self::S>) -> Size {
		ctx.sc.max
	}

	fn layout(&mut self, ctx: widget::LayoutCtx<Self::S>) -> Layout {
		ctx.suggestion
	}

	fn paint(&mut self, _ctx: &mut widget::PaintCtx<Self::S>) {}
}
