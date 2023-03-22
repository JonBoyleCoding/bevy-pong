use crate::paddle::{Paddle, PaddleSide};
use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
	pub direction: Vec2,
	pub radius: f32,
	pub position: Vec2,
	pub speed: f32,
}

impl Default for Ball {
	fn default() -> Self {
		Ball {
			direction: Vec2::new(0.0, 0.0),
			radius: 0.0,
			position: Vec2::new(0.0, 0.0),
			speed: 400.0,
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

pub fn ball_movement_system(
	mut random: ResMut<crate::random::Random>,
	mut ball_query: Query<(&mut Ball, &mut Transform)>,
	time: Res<Time>,
	win_size: Res<crate::WinSize>,
) {
	use rand::Rng;

	// Get the ball's transform
	let (mut ball, mut ball_transform) = ball_query.single_mut();

	// If ball hits the left or right of the screen, reset it
	if ball_transform.translation.x + ball.radius >= win_size.width / 2.0
		|| ball_transform.translation.x - ball.radius <= -win_size.width / 2.0
	{
		ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
		ball.direction = Vec2::new(0.0, 0.0);
	}

	// If velocity is default, set it to a random direction
	if ball.direction.x == 0.0 && ball.direction.y == 0.0 {
		let x = random.0.gen_range(-1.0..1.0);
		let y = random.0.gen_range(-1.0..1.0);

		ball.direction = Vec2::new(x, y).normalize();
		bevy::log::info!("Initial ball direction: {:?}", ball.direction);
	}

	// If ball hits the top or bottom of the screen, reverse the y direction
	if ball_transform.translation.y + ball.radius >= win_size.height / 2.0
		|| ball_transform.translation.y - ball.radius <= -win_size.height / 2.0
	{
		ball.direction.y *= -1.0;
	}

	// Calculate the velocity of the ball and move it
	ball_transform.translation += ball.direction.extend(0.0) * ball.speed * time.delta_seconds();

	// Ensure ball stays within the screen
	ball_transform.translation.y = ball_transform.translation.y.min(win_size.height / 2.0 - ball.radius);
	ball_transform.translation.y = ball_transform.translation.y.max(-win_size.height / 2.0 + ball.radius);
}

pub fn ball_paddle_hit(
	mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
	mut paddle_query: Query<(&PaddleSide, &Transform, &Sprite), With<Paddle>>,
) {
	// Get the ball's transform
	let (mut ball, mut ball_transform) = ball_query.single_mut();

	for (paddle_side, paddle_transform, paddle_sprite) in paddle_query.iter() {
		// Calculate if ball hits top 1/3, middle 1/3 or bottom 1/3 of paddle
		let paddle_height = paddle_sprite.custom_size.unwrap().y;
		let paddle_y = paddle_transform.translation.y;

		let top_third = paddle_y + (paddle_height / 3.0);
		let middle_third = paddle_y;
		let bottom_third = paddle_y - (paddle_height / 3.0);

		// Check if ball hits paddle
		if ball_transform.translation.x + ball.radius
			>= paddle_transform.translation.x - paddle_sprite.custom_size.unwrap().x / 2.0
			&& ball_transform.translation.x - ball.radius
				<= paddle_transform.translation.x + paddle_sprite.custom_size.unwrap().x / 2.0
			&& ball_transform.translation.y + ball.radius
				>= paddle_transform.translation.y - paddle_sprite.custom_size.unwrap().y / 2.0
			&& ball_transform.translation.y - ball.radius
				<= paddle_transform.translation.y + paddle_sprite.custom_size.unwrap().y / 2.0
		{
			// Reverse the x direction
			ball.direction.x *= -1.0;

			// If ball hits top 1/3 of paddle, increase y direction
			if ball_transform.translation.y + ball.radius >= top_third {
				ball.direction.y += 0.1;
			}

			// If ball hits middle 1/3 of paddle, do nothing

			// If ball hits bottom 1/3 of paddle, decrease y direction
			if ball_transform.translation.y - ball.radius <= bottom_third {
				ball.direction.y -= 0.1;
			}

			// If ball hits left paddle, increase x direction
			if paddle_side == &PaddleSide::Left {
				ball.direction.x += 0.1;
			}

			// If ball hits right paddle, decrease x direction
			if paddle_side == &PaddleSide::Right {
				ball.direction.x -= 0.1;
			}

			// Normalize the direction
			ball.direction = ball.direction.normalize();

			// Ensure ball is not stuck inside paddle
			if ball.direction.x < 0.0 {
				ball_transform.translation.x =
					paddle_transform.translation.x - paddle_sprite.custom_size.unwrap().x / 2.0 - ball.radius;
			} else {
				ball_transform.translation.x =
					paddle_transform.translation.x + paddle_sprite.custom_size.unwrap().x / 2.0 + ball.radius;
			}
		}
	}
}
