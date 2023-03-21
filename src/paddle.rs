use crate::WinSize;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use getset::{Getters, Setters};

/// The paddle side enum. This enum is used to keep track of which side the paddle is on.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Component)]
pub enum PaddleSide {
	Left,
	Right,
}

#[derive(Component)]
pub struct PlayerPaddle;

#[derive(Component)]
pub struct CPUPaddle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
	CPU,
	Human,
}

#[derive(Resource)]
pub struct PaddleConfig {
	pub player_types: [PlayerType; 2],
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
		Paddle { y_pos: 50.0, size: 0.0 }
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

pub fn paddle_spawn_system(mut commands: Commands, win_size: Res<crate::WinSize>, paddle_config: Res<PaddleConfig>) {
	use crate::paddle::*;
	use bevy::ecs::system::EntityCommands;

	// Paddle sprite
	let paddle_sprite = Sprite {
		custom_size: Some(Vec2::new(10.0, 100.0)),
		color: Color::rgb(1.0, 1.0, 1.0),
		..Default::default()
	};

	// Spawn the left paddle
	let mut paddle_left = commands.spawn((
		PaddleSide::Left,
		Paddle::new(10.0),
		SpriteBundle {
			sprite: paddle_sprite.clone(),
			transform: Transform::from_xyz(-(win_size.width / 2.0 - 10.0), 0.0, 0.0),
			..Default::default()
		},
	));

	// Add the PlayerPaddle or CPUPaddle component depending on the player type
	let add_paddle_type = |paddle: &mut EntityCommands, paddle_side: PaddleSide| {
		if paddle_config.player_types[paddle_side as usize] == PlayerType::Human {
			paddle.insert(PlayerPaddle);
		} else if paddle_config.player_types[paddle_side as usize] == PlayerType::CPU {
			paddle.insert(CPUPaddle);
		}
	};

	add_paddle_type(&mut paddle_left, PaddleSide::Left);

	// Spawn the right paddle
	let mut paddle_right = commands.spawn((
		PaddleSide::Right,
		Paddle::new(10.0),
		SpriteBundle {
			sprite: paddle_sprite,
			transform: Transform::from_xyz((win_size.width / 2.0 - 10.0), 0.0, 0.0),
			..Default::default()
		},
	));

	add_paddle_type(&mut paddle_right, PaddleSide::Right);
}

pub fn paddle_human_movement_system(
	keyboard_input: Res<Input<KeyCode>>,
	win_size: Res<WinSize>,
	mut paddle_query: Query<(&PaddleSide, &mut Paddle, &mut Transform), With<(PlayerPaddle)>>,
) {
	// Y pos of the paddle (0 - 100) maps to the y pos of the paddle (-win_size.height / 2.0 - win_size.height / 2.0)
	let y_pos_to_y = |y_pos: f32| ((y_pos / 100.0) * win_size.height) - (win_size.height / 2.0);

	for (paddle_side, mut paddle, mut transform) in paddle_query.iter_mut() {
		let mut y_pos = *paddle.y_pos();

		// Move the paddle up or down depending on the key pressed
		match *paddle_side {
			PaddleSide::Left => {
				if keyboard_input.pressed(KeyCode::W) {
					y_pos += 1.0;
				} else if keyboard_input.pressed(KeyCode::S) {
					y_pos -= 1.0;
				}
			}
			PaddleSide::Right => {
				if keyboard_input.pressed(KeyCode::Up) {
					y_pos += 1.0;
				} else if keyboard_input.pressed(KeyCode::Down) {
					y_pos -= 1.0;
				}
			}
		}

		// Clamp the y pos to 0 - 100
		y_pos = y_pos.clamp(0.0, 100.0);

		// Update the paddle's y pos
		paddle.set_y_pos(y_pos);

		// Update the paddle's transform
		transform.translation.y = y_pos_to_y(y_pos);
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
