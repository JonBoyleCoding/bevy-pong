use bevy::prelude::*;
use getset::{Getters, Setters};

/// The paddle side enum. This enum is used to keep track of which side the paddle is on.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Component)]
pub enum PaddleSide {
	Left,
	Right,
}

/// The paddle component. This component is used to keep track of the paddle's position, size, and side.
#[derive(Component, Getters, Setters)]
pub struct Paddle {
	#[getset(get = "pub", set = "pub")]
	y_pos: f32,
	#[getset(get = "pub", set = "pub")]
	size: f32,
}

impl Default for Paddle {
	fn default() -> Self {
		Paddle { y_pos: 0.0, size: 0.0 }
	}
}

impl Paddle {
	pub fn new(size: f32) -> Self {
		Paddle {
			size,
			..Default::default()
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn paddle_new() {
		let paddle = Paddle::new(10.0);
		assert_eq!(*paddle.size(), 10.0);
		assert_eq!(*paddle.y_pos(), 0.0);
	}
}
