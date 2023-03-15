use bevy::prelude::*;
use getset::{Getters, Setters};

/// The paddle side enum. This enum is used to keep track of which side the paddle is on.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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
    #[getset(get = "pub", set = "pub")]
    side: PaddleSide,
}

impl Default for Paddle {
    fn default() -> Self {
        Paddle {
            y_pos: 0.0,
            size: 0.0,
            side: PaddleSide::Left,
        }
    }
}

impl Paddle {
    pub fn new(side: PaddleSide, size: f32) -> Self {
        Paddle {
            side,
            size,
            ..Default::default()
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn paddle_new() {
        let paddle = Paddle::new(PaddleSide::Left, 10.0);
        assert_eq!(*paddle.side(), PaddleSide::Left);
        assert_eq!(*paddle.size(), 10.0);
        assert_eq!(*paddle.y_pos(), 0.0);
    }

}
