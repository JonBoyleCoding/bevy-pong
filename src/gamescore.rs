use bevy::prelude::*;
use crate::paddle::PaddleSide;
use getset::{Getters};

const DEFAULT_WIN_SCORE: u32 = 10;

#[derive(Resource, Getters)]
pub struct GameScore {
    #[getset(get = "pub")]
    score: [u32; 2],
    #[getset(get = "pub")]
    win_score: u32,
}

impl Default for GameScore {
    fn default() -> Self {
        GameScore {
            score: [0, 0],
            win_score: DEFAULT_WIN_SCORE,
        }
    }
}

impl GameScore {
    /// Creates a new game score.
    ///
    /// # Arguments
    /// * `win_score` - The score needed to win the game.
    ///
    pub fn new(win_score: u32) -> Self {
        GameScore {
            win_score,
            ..Default::default()
        }
    }

    /// Returns the score for the given side.
    ///
    /// # Arguments
    /// * `side` - The side to get the score for.
    ///
    /// # Returns
    /// The score for the given side.
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
    /// use bevy_pong::gamescore::GameScore;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_score = GameScore::default();
    /// assert_eq!(game_score.get_score(PaddleSide::Left), 0);
    /// assert_eq!(game_score.get_score(PaddleSide::Right), 0);
    ///
    /// game_score.increment_score(PaddleSide::Left);
    /// assert_eq!(game_score.get_score(PaddleSide::Left), 1);
    /// assert_eq!(game_score.get_score(PaddleSide::Right), 0);
    ///
    /// game_score.increment_score(PaddleSide::Right);
    /// assert_eq!(game_score.get_score(PaddleSide::Left), 1);
    /// assert_eq!(game_score.get_score(PaddleSide::Right), 1);
    /// ```
    pub fn increment_score(&mut self, side: PaddleSide) {
        self.score[side as usize] += 1;
    }

    /// Resets the score to 0 for both sides.
    pub fn reset_score(&mut self) {
        self.score = [0, 0];
    }

    /// Returns the winner if there is one.
    ///
    /// # Returns
    /// The winner if there is one. Otherwise, `None`.
    ///
    /// # Example
    /// ```
    /// use bevy_pong::gamescore::GameScore;
    /// use bevy_pong::paddle::PaddleSide;
    ///
    /// let mut game_score = GameScore::default();
    /// assert_eq!(game_score.get_winner(), None);
    ///
    /// for _ in 0..DEFAULT_WIN_SCORE {
    ///    game_score.increment_score(PaddleSide::Left);
    /// }
    /// assert_eq!(game_score.get_winner(), Some(PaddleSide::Left));
    /// ```
    pub fn get_winner(&self) -> Option<PaddleSide> {
        if self.score[PaddleSide::Left as usize] >= self.win_score {
            Some(PaddleSide::Left)
        } else if self.score[PaddleSide::Right as usize] >= self.win_score {
            Some(PaddleSide::Right)
        } else {
            None
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_score_new() {
        let game_score = GameScore::new(5);
        assert_eq!(game_score.get_score(PaddleSide::Left), 0);
        assert_eq!(game_score.get_score(PaddleSide::Right), 0);
        assert_eq!(game_score.win_score, 5);
    }

    #[test]
    fn win_left() {
        let mut game_score = GameScore::default();
        for _ in 0..DEFAULT_WIN_SCORE {
            game_score.increment_score(PaddleSide::Left);
        }
        assert_eq!(game_score.get_winner(), Some(PaddleSide::Left));
    }

    #[test]
    fn win_right() {
        let mut game_score = GameScore::default();
        for _ in 0..DEFAULT_WIN_SCORE {
            game_score.increment_score(PaddleSide::Right);
        }
        assert_eq!(game_score.get_winner(), Some(PaddleSide::Right));
    }

    #[test]
    fn no_win() {
        let mut game_score = GameScore::default();
        for _ in 0..DEFAULT_WIN_SCORE - 1 {
            game_score.increment_score(PaddleSide::Left);
        }
        assert_eq!(game_score.get_winner(), None);
    }

    #[test]
    fn custom_win() {
        let mut game_score = GameScore::new(3);
        for _ in 0..3 {
            game_score.increment_score(PaddleSide::Left);
        }
        assert_eq!(game_score.get_winner(), Some(PaddleSide::Left));
    }

    #[test]
    fn reset_score() {
        let mut game_score = GameScore::default();
        for _ in 0..DEFAULT_WIN_SCORE {
            game_score.increment_score(PaddleSide::Left);
        }
        assert_eq!(game_score.get_winner(), Some(PaddleSide::Left));
        game_score.reset_score();
        assert_eq!(game_score.get_winner(), None);
        assert_eq!(game_score.get_score(PaddleSide::Left), 0);
        assert_eq!(game_score.get_score(PaddleSide::Right), 0);
    }
}
