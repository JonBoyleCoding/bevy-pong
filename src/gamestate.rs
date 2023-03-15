use bevy::prelude::*;
use crate::paddle::{PaddleSide, Paddle};
use crate::ball::Ball;

const WIN_SCORE: u32 = 10;
const PADDLE_DEFAULT_SIZE: f32 = 30.0;

/// The game state resource. This resource is used to keep track of the score
#[derive(Resource)]
pub struct GameState {
    score: [u32; 2],
    paddles: [Paddle; 2],
    ball: Ball,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: [0, 0],
            paddles: [Paddle::new(PaddleSide::Left, PADDLE_DEFAULT_SIZE), Paddle::new(PaddleSide::Right, PADDLE_DEFAULT_SIZE)],
            ball: Ball::new(5.0, Vec2::new(0.0, 0.0)),
        }
    }
}

impl GameState {

    /// Creates a new game state.
    ///
    /// # Arguments
    /// * `paddle_size` - The size of the paddles.
    /// * `ball_radius` - The radius of the ball.
    /// * `ball_position` - The position of the ball.
    ///
    /// # Returns
    /// A new game state.
    pub fn new(paddle_size: f32, ball_radius: f32, ball_position: Vec2) -> Self {
        GameState {
            score: [0, 0],
            paddles: [Paddle::new(PaddleSide::Left, paddle_size), Paddle::new(PaddleSide::Right, paddle_size)],
            ball: Ball::new(ball_radius, ball_position),
        }
    }

    /// Returns the score for the given side.
    ///
    /// # Arguments
    /// * `side` - The side to get the score for.
    ///
    /// # Returns
    /// The score for the given side.
    ///
    /// # Example
    /// ```
    /// use bevy_pong::gamestate::GameState;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_state = GameState::default();
    /// assert_eq!(game_state.get_score(PaddleSide::Left), 0);
    /// assert_eq!(game_state.get_score(PaddleSide::Right), 0);
    /// ```
    pub fn get_score(&self, side: PaddleSide) -> u32 {
        self.score[side as usize]
    }

    /// Increments the score for the given side.
    ///
    /// # Arguments
    /// * `side` - The side to increment the score for.
    ///
    /// # Example
    /// ```
    /// use bevy_pong::gamestate::GameState;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_state = GameState::default();
    /// game_state.increment_score(PaddleSide::Left);
    /// assert_eq!(game_state.get_score(PaddleSide::Left), 1);
    /// ```
    pub fn increment_score(&mut self, side: PaddleSide) {
        self.score[side as usize] += 1;
    }

    /// Resets the score to 0 for both sides.
    ///
    /// # Example
    /// ```
    /// use bevy_pong::gamestate::GameState;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_state = GameState::default();
    /// game_state.increment_score(PaddleSide::Left);
    /// game_state.increment_score(PaddleSide::Right);
    /// assert_eq!(game_state.get_score(PaddleSide::Left), 1);
    /// assert_eq!(game_state.get_score(PaddleSide::Right), 1);
    /// game_state.reset_score();
    /// assert_eq!(game_state.get_score(PaddleSide::Left), 0);
    /// assert_eq!(game_state.get_score(PaddleSide::Right), 0);
    /// ```
    pub fn reset_score(&mut self) {
        self.score = [0, 0];
    }

    /// Returns the side that won the game, if any.
    ///
    /// # Returns
    /// The side that won the game, if any. If no side has won the game, `None` is returned.
    ///
    /// # Example
    /// ```
    /// use bevy_pong::gamestate::GameState;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_state = GameState::default();
    /// assert_eq!(game_state.side_won(), None);
    ///
    /// for _ in 0..WIN_SCORE-1 {
    ///    game_state.increment_score(PaddleSide::Left);
    ///    assert_eq!(game_state.side_won(), None);
    /// }
    ///
    /// game_state.increment_score(PaddleSide::Left);
    /// assert_eq!(game_state.side_won(), Some(PaddleSide::Left));
    /// ```
    pub fn side_won(&self) -> Option<PaddleSide> {

        if self.score[0] >= WIN_SCORE {
            Some(PaddleSide::Left)
        } else if self.score[1] >= WIN_SCORE {
            Some(PaddleSide::Right)
        } else {
            None
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_get_score() {
        let mut game_state = GameState::default();
        assert_eq!(game_state.get_score(PaddleSide::Left), 0);
        assert_eq!(game_state.get_score(PaddleSide::Right), 0);
        game_state.increment_score(PaddleSide::Left);
        assert_eq!(game_state.get_score(PaddleSide::Left), 1);
        assert_eq!(game_state.get_score(PaddleSide::Right), 0);
        game_state.increment_score(PaddleSide::Right);
        assert_eq!(game_state.get_score(PaddleSide::Left), 1);
        assert_eq!(game_state.get_score(PaddleSide::Right), 1);
    }

    #[test]
    fn test_win_left() {
        let mut game_state = GameState::default();
        for _ in 0..WIN_SCORE {
            game_state.increment_score(PaddleSide::Left);
        }
        assert_eq!(game_state.side_won(), Some(PaddleSide::Left));
    }

    #[test]
    fn test_win_right() {
        let mut game_state = GameState::default();
        for _ in 0..WIN_SCORE {
            game_state.increment_score(PaddleSide::Right);
        }
        assert_eq!(game_state.side_won(), Some(PaddleSide::Right));
    }
}
