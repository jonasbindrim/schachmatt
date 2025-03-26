use pest::{
    Parser,
    iterators::{Pair, Pairs},
};

use crate::{
    Game, Position, SAN,
    util::error::{
        error_messages::{ILLEGAL_TURN_ERROR, PGN_IMPORT_ERROR},
        parser_error::ParserError,
    },
};

#[derive(Parser)]
#[grammar = "util/pest_definitions/portable_game_notation.pest"]
struct PgnStruct;

/// Converts a string in PGN representation into a `Game`.
/// - `pgn_string` - The string containing pgn data
/// - `callback_option` - A callback function used for custom pgns
/// - `returns` - A game or an error
/// # Errors
/// Returns a `ParserError` when the PGN string cannot be converted into a `Game`.
pub fn game_from_pgn(pgn_string: &str) -> Result<Game, ParserError> {
    let pgn_lines: Vec<&str> = pgn_string.lines().collect();
    let mut game = Game::new_empty();

    // Find the index of the line which seperates metadata and gamedata
    let part_seperator = match find_empty_line(&pgn_lines) {
        Ok(index) => index,
        Err(_) => pgn_lines.len(),
    };

    if let Option::Some(error) = parse_metadata_lines(&pgn_lines[0..part_seperator], &mut game) {
        return Err(error);
    }

    // Handle turn data
    game.push_position(Position::new());
    let mut turn_data = String::new();
    for line in &pgn_lines[part_seperator + 1..] {
        turn_data.push_str(line);
        turn_data.push(' ');
    }

    match PgnStruct::parse(Rule::turn_data, &turn_data) {
        Ok(pairs) => {
            let turn_data_pair = pairs.collect::<Vec<Pair<Rule>>>()[0].clone();
            for pair in turn_data_pair.into_inner() {
                if matches!(
                    pair.as_rule(),
                    Rule::single_move_entry | Rule::two_move_entry
                ) {
                    handle_move_entry(pair.into_inner(), &mut game)?
                }
            }
        }
        Err(_) => return Err(ParserError::new(PGN_IMPORT_ERROR)),
    }

    Ok(game)
}

/// Handles the parsing of a single move entry
fn handle_move_entry(pairs: Pairs<Rule>, game: &mut Game) -> Result<(), ParserError> {
    for pair in pairs {
        if matches!(pair.as_rule(), Rule::turn_move) {
            let turn = pair
                .into_inner()
                .find(|pair| matches!(pair.as_rule(), Rule::san_move))
                .unwrap();

            let turn = SAN::import(turn.as_str(), &mut game.get_current_state())?;
            if game.execute_turn(turn).is_err() {
                return Err(ParserError::new(ILLEGAL_TURN_ERROR));
            }
        }
    }

    Ok(())
}

/// Finds and returns the index of the first empty line.
/// The first empty line in a pgn is the seperator between the metadata and the turn data
/// - `lines` - The pgn data lines
/// - `returns` - A result containing a line index or an error
fn find_empty_line(lines: &[&str]) -> Result<usize, ParserError> {
    for line_index in 0..lines.len() {
        if lines.get(line_index).unwrap().is_empty() {
            return Ok(line_index);
        }
    }
    Err(ParserError::new(PGN_IMPORT_ERROR))
}

/// Converts each given line into a key-value pair and adds it as metadata to the game
/// - `lines` - The metadata lines of a pgn string
/// - `game` - The game to which the metadata is appended
/// - `returns` - An optional error
fn parse_metadata_lines(lines: &[&str], game: &mut Game) -> Option<ParserError> {
    for line in lines {
        // Handle a single line
        let mut key = String::new();
        match PgnStruct::parse(Rule::metadata, line) {
            Ok(pairs) => {
                for rules in pairs.collect::<Vec<Pair<Rule>>>()[0].clone().into_inner() {
                    match rules.as_rule() {
                        Rule::metadata_key => key = rules.as_str().to_string(),
                        Rule::metadata_value => {
                            game.set_metadata(&key, rules.as_str());
                        }
                        _ => return Some(ParserError::new(PGN_IMPORT_ERROR)),
                    }
                }
            }
            Err(_) => return Some(ParserError::new(PGN_IMPORT_ERROR)),
        }
    }
    None
}
