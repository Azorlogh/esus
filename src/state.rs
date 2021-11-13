pub trait State: std::fmt::Debug {
	type Message: Clone + std::fmt::Debug;
}
