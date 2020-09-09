pub fn init_simple_logger() {
	simple_logger::init_with_level(log::Level::Warn).expect("Failed to init simple logger");
}

pub use kurbo::{Rect, Size};

pub mod device;
pub mod event;
pub mod instance;
pub mod native_widgets;
mod painter;
mod render;
mod util;
pub mod widget;

mod data;
pub use data::*;
pub use std::f64::INFINITY as INF;
