#![allow(unused, dead_code)]

mod ball;
mod gamescore;
mod paddle;

use crate::paddle::{paddle_human_movement_system, paddle_spawn_system, PaddleConfig, PlayerType};
use bevy::app::SystemAppConfig;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

#[derive(Resource)]
pub struct WinSize {
	pub width: f32,
	pub height: f32,
}

impl Default for WinSize {
	fn default() -> Self {
		WinSize {
			width: -1.0,
			height: -1.0,
		}
	}
}

fn main() {
	let player_types = PaddleConfig {
		player_types: [PlayerType::Human, PlayerType::CPU],
	};

	App::new()
		// Set Clear Colour to Black
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.insert_resource(WinSize::default())
		.insert_resource(player_types)
		// Add the DefaultPlugins, which contains all the basic plugins, with the WindowPlugin to set the window title and size
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Pong".to_string(),
				resolution: WindowResolution::new(800.0, 600.0),
				resizable: false,
				..Default::default()
			}),
			..Default::default()
		}))
		// Add the setup_system to the startup stage
		.add_startup_system(setup_system.before(paddle_spawn_system))
		.add_startup_system(paddle_spawn_system)
		.add_system(paddle_human_movement_system)
		// Run the game
		.run();
}

fn setup_system(
	mut commands: Commands,
	mut window_size: ResMut<WinSize>,
	primary_window: Query<&Window, With<PrimaryWindow>>,
) {
	// Add a 2D camera
	commands.spawn(Camera2dBundle::default());

	// Add the GameScore resource (tracks the score)
	use gamescore::*;
	commands.insert_resource(GameScore::default());

	let window = primary_window.get_single().unwrap();
	window_size.width = window.width();
	window_size.height = window.height();

	//////////////////
	// Add the Ball
	//////////////////
	use ball::Ball;

	// Ball sprite
	let ball_diameter = 10.0;
	let ball_sprite = Sprite {
		custom_size: Some(Vec2::new(ball_diameter, ball_diameter)),
		color: Color::rgb(1.0, 1.0, 1.0),
		..Default::default()
	};

	// Spawn the ball
	commands.spawn((
		Ball::new(
			ball_diameter / 2.0,
			Vec2::new((window_size.width / 2.0), (window_size.height / 2.0)),
		),
		SpriteBundle {
			sprite: ball_sprite,
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..Default::default()
		},
	));
}
