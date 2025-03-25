use crate::{Game, GameResult, PlayerColor, SAN};

static REQUIRED_TAGS: [&str; 6] = ["Event", "Site", "Date", "Round", "White", "Black"];
static RESULT_TAG: &str = "Result";

/// Converts a `Game` into its PGN representation.
/// - `game` - The game which is converted
/// - `returns` - A string representing the game in pgn form
#[must_use]
pub fn game_to_pgn(game: &Game) -> String {
    let mut output_game = game.clone();

    add_seven_tag_roster(&mut output_game);

    let metadata = format_metadata(&output_game);
    let turndata = format_turndata(&mut output_game);
    format!("{}\n{}", metadata, turndata)
}

/// Formats the turn data of the given game into pgn format
/// - `game` - The game containing the turns that will be added
/// - `returns` - The formatted turn data output
fn format_turndata(game: &mut Game) -> String {
    let mut result = String::new();
    let mut first_fullmove_indicator: bool = true;

    for position_index in 0..game.position_history.len() {
        let position = game.get_state_at_index_reference(position_index);

        // Add game result. Only done once in the last position
        if position_index == game.position_history.len() - 1 {
            let game_result = match position.game_over_check() {
                Some(result) => match result {
                    GameResult::Draw => "1/2-1/2",
                    GameResult::Over(player_color) => match player_color {
                        PlayerColor::Black => "0-1",
                        PlayerColor::White => "1-0",
                    },
                },
                None => "*",
            };
            game.set_metadata("Result", game_result);
            result.push_str(game_result);
            break;
        }

        // Print fullmove counter
        match position.active_color {
            PlayerColor::Black => {
                if first_fullmove_indicator {
                    result.push_str(&position.fullmove_counter.to_string());
                    result.push_str(".. ");
                    first_fullmove_indicator = false;
                }
            }
            PlayerColor::White => {
                result.push_str(&position.fullmove_counter.to_string());
                result.push_str(". ");
                first_fullmove_indicator = false;
            }
        }

        // Add turn san data
        let turn = game.get_turn_at_index(position_index);
        result.push_str(&SAN::export(turn, position));
        result.push(' ');
    }

    result
}

/// Formats the metadata of the given game into pgn format
/// - `game` - The game containing the metadata that gets appended
/// - `returns` - The formatted metadata output
fn format_metadata(game: &Game) -> String {
    let metadata_map = game.get_metadata_map();
    metadata_map
        .iter()
        .map(|(key, value)| format!("[{} \"{}\"]\n", key, value))
        .collect::<Vec<String>>()
        .join("")
}

/// Adds the required metadata entries of the game if not set already
fn add_seven_tag_roster(game: &mut Game) {
    for tag in REQUIRED_TAGS {
        if game.get_metadata(tag).is_none() {
            game.set_metadata(tag, "");
        }
    }

    if game.get_metadata(RESULT_TAG).is_some() {
        return;
    }

    let result = match game.get_game_result() {
        Some(game_result) => match game_result {
            GameResult::Over(player_color) => match player_color {
                PlayerColor::Black => "0-1",
                PlayerColor::White => "1-0",
            },
            GameResult::Draw => "1/2-1/2",
        },
        None => "*",
    };

    game.set_metadata(RESULT_TAG, result);
}
