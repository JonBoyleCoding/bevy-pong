use crate::WinSize;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use getset::{Getters, Setters};

/// The paddle side enum. This enum is used to keep track of which side the paddle is on.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Component, strum::Display)]
pub enum PaddleSide {
	Left,
	Right,
}

#[derive(Component)]
pub struct PlayerPaddle;

#[derive(Component)]
pub struct CPUPaddle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
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
	#[getset(get = "pub", set = "pub")]
	speed: f32,
}

impl Default for Paddle {
	fn default() -> Self {
		Paddle {
			y_pos: 50.0,
			size: 0.0,
			speed: 50.0,
		}
	}
}

impl Paddle {
	pub fn new(size: f32, speed: f32) -> Self {
		Paddle {
			size,
			speed,
			..Default::default()
		}
	}

	pub fn new_default_size(speed: f32) -> Self {
		Paddle {
			speed,
			..Default::default()
		}
	}

	pub fn new_default_speed(size: f32) -> Self {
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
		Paddle::new(100.0, 50.0),
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
		Paddle::new(100.0, 50.0),
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
	time: Res<Time>,
	mut paddle_query: Query<(&PaddleSide, &mut Paddle, &mut Transform), With<(PlayerPaddle)>>,
) {
	// Y pos of the paddle (0 - 100) maps to the y pos of the paddle (-win_size.height / 2.0 - win_size.height / 2.0)
	// and takes into account the paddle's size with position being the center of the paddle
	let y_pos_to_y = |y_pos: f32, p_size: f32| {
		((y_pos / 100.0) * (win_size.height - p_size)) - (win_size.height / 2.0 - p_size / 2.0)
	};

	for (paddle_side, mut paddle, mut transform) in paddle_query.iter_mut() {
		let mut y_pos = *paddle.y_pos();

		let move_paddle = |y_pos: &mut f32, up: bool, down: bool| {
			if up {
				*y_pos += paddle.speed() * time.delta_seconds();
			} else if down {
				*y_pos -= paddle.speed() * time.delta_seconds();
			}
		};

		// Move the paddle up or down depending on the key pressed
		match *paddle_side {
			PaddleSide::Left => {
				move_paddle(
					&mut y_pos,
					keyboard_input.pressed(KeyCode::W),
					keyboard_input.pressed(KeyCode::S),
				);
			}
			PaddleSide::Right => {
				move_paddle(
					&mut y_pos,
					keyboard_input.pressed(KeyCode::Up),
					keyboard_input.pressed(KeyCode::Down),
				);
			}
		}

		// Clamp the y pos to 0 - 100
		y_pos = y_pos.clamp(0.0, 100.0);

		// Update the paddle's y pos
		paddle.set_y_pos(y_pos);

		// Update the paddle's transform
		transform.translation.y = y_pos_to_y(y_pos, *paddle.size());
	}
}

pub fn paddle_cpu_movement_system(
	ball_query: Query<&Transform, (With<crate::ball::Ball>, Without<Paddle>)>,
	win_size: Res<WinSize>,
	time: Res<Time>,
	mut paddle_query: Query<(&PaddleSide, &mut Paddle, &mut Transform), With<(CPUPaddle)>>,
) {
	// Y pos of the paddle (0 - 100) maps to the y pos of the paddle (-win_size.height / 2.0 - win_size.height / 2.0)
	// and takes into account the paddle's size with position being the center of the paddle
	let y_pos_to_y = |y_pos: f32, p_size: f32| {
		((y_pos / 100.0) * (win_size.height - p_size)) - (win_size.height / 2.0 - p_size / 2.0)
	};

	for (paddle_side, mut paddle, mut transform) in paddle_query.iter_mut() {
		let mut y_pos = *paddle.y_pos();

		// Get the ball's transform
		let ball_transform = ball_query.single();

		let approach_ball = |y_pos: &mut f32, ball_transform: &Transform| {
			if ball_transform.translation.y > y_pos_to_y(*y_pos, *paddle.size()) {
				*y_pos += paddle.speed() * time.delta_seconds();
			} else if ball_transform.translation.y < y_pos_to_y(*y_pos, *paddle.size()) {
				*y_pos -= paddle.speed() * time.delta_seconds();
			}
		};

		// Move the paddle up or down depending on the ball's position
		match *paddle_side {
			PaddleSide::Left => {
				if ball_transform.translation.x < 0.0 {
					approach_ball(&mut y_pos, &ball_transform);
				}
			}
			PaddleSide::Right => {
				if ball_transform.translation.x > 0.0 {
					approach_ball(&mut y_pos, &ball_transform);
				}
			}
		}

		// Clamp the y pos to 0 - 100
		y_pos = y_pos.clamp(0.0, 100.0);

		// Update the paddle's y pos
		paddle.set_y_pos(y_pos);

		// Update the paddle's transform
		transform.translation.y = y_pos_to_y(y_pos, *paddle.size());
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn paddle_new() {
		let paddle = Paddle::new(10.0, 5.0);
		assert_eq!(*paddle.size(), 10.0);
		assert_eq!(*paddle.y_pos(), 0.0);
		assert_eq!(*paddle.speed(), 5.0);
	}
}
