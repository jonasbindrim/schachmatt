use crate::PlayerColor;

/// Represents the result of a chess game. An ongoing game is not considered a result.
/// The game can end in a draw or with one of the players winning.
#[derive(PartialEq, Debug)]
pub enum GameResult {
    /// The game ended in a draw.
    Draw,
    /// The game is over with `PlayerColor` being the winner.
    Over(PlayerColor),
}

impl GameResult {
    /// Returns the string representation of the game result.
    /// If the game is ongoing, it returns `*`.
    pub fn to_string(result: Option<&Self>) -> String {
        let Some(result) = result else {
            return String::from("*");
        };

        match result {
            GameResult::Draw => String::from("1/2-1/2"),
            GameResult::Over(player_color) => match player_color {
                PlayerColor::Black => String::from("0-1"),
                PlayerColor::White => String::from("1-0"),
            },
        }
    }

    /// Converts a string representation of the game result to a `GameResult` enum.
    /// Returns `None` if the string is not a valid representation of a game result.
    pub fn from_string(result: &str) -> Option<GameResult> {
        match result {
            "1-0" => Some(Self::Over(PlayerColor::White)),
            "0-1" => Some(Self::Over(PlayerColor::Black)),
            "1/2-1/2" => Some(Self::Draw),
            _ => None,
        }
    }
}
