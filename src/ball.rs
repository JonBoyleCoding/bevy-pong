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

	let (mut ball, mut ball_transform) = ball_query.single_mut();

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

	ball_transform.translation += ball.direction.extend(0.0) * ball.speed * time.delta_seconds();
}
