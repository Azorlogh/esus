pub fn init_simple_logger() {
	simple_logger::init_with_level(log::Level::Warn).expect("Failed to init simple logger");
}

pub use lyon::math::{Point, Rect, Size, Vector};

pub mod device;
pub mod event;
pub mod instance;
pub mod native_widgets;
mod painter;
mod render;
pub mod state;
pub use state::State;
mod util;
pub mod widget;

mod data;
pub use data::*;
pub use std::f64::INFINITY as INF;
