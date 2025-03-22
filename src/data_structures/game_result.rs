use super::player_color::PlayerColor;

/// Represents the result of a chess game.
#[derive(PartialEq)]
pub enum GameResult {
    /// The game ended in a draw.
    Draw,
    /// The game has not ended yet.
    None,
    /// The game is over with `PlayerColor` being the winner.
    Over(PlayerColor),
}
