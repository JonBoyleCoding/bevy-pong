use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
	pub velocity: Vec2,
	pub radius: f32,
	pub position: Vec2,
}

impl Default for Ball {
	fn default() -> Self {
		Ball {
			velocity: Vec2::new(0.0, 0.0),
			radius: 0.0,
			position: Vec2::new(0.0, 0.0),
		}
	}
}

impl Ball {
	pub fn new(radius: f32, position: Vec2) -> Self {
		Ball {
			radius,
			position,
			..Default::default()
		}
	}
}
