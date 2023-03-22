use bevy::prelude::*;
use rand::{Rng, SeedableRng};

#[derive(Resource)]
pub struct Random(pub rand::rngs::StdRng);

impl Default for Random {
	fn default() -> Self {
		Random(rand::rngs::StdRng::from_entropy())
	}
}

impl Random {
	pub fn new_with_seed(seed: u64) -> Self {
		Random(rand::rngs::StdRng::seed_from_u64(seed))
	}
}
