use super::player_color::PlayerColor;

/// Represents the result of a chess game.
#[derive(PartialEq)]
pub enum GameResult {
    /// The game ended in a draw.
    Draw,
    /// The game is over with `PlayerColor` being the winner.
    Over(PlayerColor),
}

impl GameResult {
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

    pub fn from_string(result: &str) -> Option<GameResult> {
        match result {
            "1-0" => Some(Self::Over(PlayerColor::White)),
            "0-1" => Some(Self::Over(PlayerColor::Black)),
            "1/2-1/2" => Some(Self::Draw),
            _ => None,
        }
    }
}
