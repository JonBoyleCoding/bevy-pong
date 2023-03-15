#![allow(unused, dead_code)]

mod ball;
mod gamescore;
mod paddle;

use crate::paddle::{Paddle, PaddleSide};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

fn main() {
	App::new()
		// Set Clear Colour to Black
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
		.add_startup_system(setup_system)
		// Run the game
		.run();
}

fn setup_system(mut commands: Commands, mut primary_window: Query<&Window, With<PrimaryWindow>>) {
	// Add a 2D camera
	commands.spawn(Camera2dBundle::default());

	// Add the GameScore resource (tracks the score)
	use gamescore::*;
	commands.insert_resource(GameScore::default());

	let window = primary_window.get_single().unwrap();
	let window_size = Vec2::new(window.width(), window.height());

	//////////////////
	// Add the Paddles
	//////////////////
	use paddle::*;

	// Paddle sprite
	let paddle_sprite = Sprite {
		custom_size: Some(Vec2::new(10.0, 100.0)),
		color: Color::rgb(1.0, 1.0, 1.0),
		..Default::default()
	};

	// Spawn the left paddle
	commands.spawn((
		PaddleSide::Left,
		Paddle::new(10.0),
		SpriteBundle {
			sprite: paddle_sprite.clone(),
			transform: Transform::from_translation(Vec3::new(-(window_size.x / 2.0 - 10.0), 0.0, 0.0)),
			..Default::default()
		},
	));

	// Spawn the right paddle
	commands.spawn((
		PaddleSide::Right,
		Paddle::new(10.0),
		SpriteBundle {
			sprite: paddle_sprite,
			transform: Transform::from_translation(Vec3::new((window_size.x / 2.0 - 10.0), 0.0, 0.0)),
			..Default::default()
		},
	));

	//////////////////
	// Add the Ball
	//////////////////
	use ball::*;

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
			Vec2::new((window_size.x / 2.0), (window_size.y / 2.0)),
		),
		SpriteBundle {
			sprite: ball_sprite,
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..Default::default()
		},
	));
}
