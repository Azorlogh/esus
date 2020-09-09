use std::num::NonZeroU64;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Id(NonZeroU64);

impl Id {
	pub fn initial() -> Id {
		Id(NonZeroU64::new(1).unwrap())
	}

	pub fn new_unchecked(x: u64) -> Id {
		Id(NonZeroU64::new(x).expect("tried to create 0 id"))
	}

	pub fn next(&mut self) -> Id {
		let next = NonZeroU64::new(self.0.get() + 1).unwrap(); // safe
		self.0 = next;
		Id(next)
	}
}
