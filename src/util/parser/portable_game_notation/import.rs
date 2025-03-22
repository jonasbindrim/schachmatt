use pest::{Parser, iterators::Pair};

use crate::{
    Game, Position, SAN,
    util::error::{error_messages::PGN_IMPORT_ERROR, parser_error::ParserError},
};

#[derive(Parser)]
#[grammar = "util/parser/portable_game_notation/pgn.pest"]
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

    game.push_position(Position::new());
    let mut turn_data = String::new();
    for line in &pgn_lines[part_seperator + 1..] {
        turn_data.push_str(line);
        turn_data.push(' ');
    }

    match PgnStruct::parse(Rule::full_turn_data, &turn_data) {
        Ok(pairs) => {
            let pair = pairs.collect::<Vec<Pair<Rule>>>()[0].clone();
            match handle_full_turn_data(pair, &mut game) {
                Some(err) => Err(err),
                None => Ok(game),
            }
        }
        Err(_) => Err(ParserError::new(PGN_IMPORT_ERROR)),
    }
}

/// Converts the complete turn data string of a pgn into the corresponding moves and positions
/// - `pair` - A pest pair containing the full turn data part
/// - `game` - The game which will store the converted pgn data
/// - `returns` - An optional error
fn handle_full_turn_data(pair: Pair<Rule>, game: &mut Game) -> Option<ParserError> {
    match pair.as_rule() {
        Rule::full_turn_data => {
            let pairs = pair.into_inner();
            for single_pair in pairs {
                if let Some(error) = handle_full_turn_data(single_pair, game) {
                    return Some(error);
                }
            }
        }
        Rule::turn_data => {
            if let Some(error) = handle_turn_data(pair, game) {
                return Some(error);
            }
        }
        _ => return Some(ParserError::new(PGN_IMPORT_ERROR)),
    }
    None
}

/// Converts a single turn data block
/// - `pair` - Pest data containing a single turn
/// - `game` - The game to which the turn is appended
fn handle_turn_data(pair: Pair<Rule>, game: &mut Game) -> Option<ParserError> {
    // By definition the turn data rule can only be composed of a single sub rules
    let binding = pair.into_inner().collect::<Vec<Pair<Rule>>>();
    let inner_rule = binding.first().unwrap();

    if inner_rule.as_rule() == Rule::san_move {
        let san_string = inner_rule.as_str();
        let turn_option = SAN::import(san_string, &mut game.get_current_state());

        match turn_option {
            Some(turn) => game.execute_turn(turn),
            None => return Some(ParserError::new(PGN_IMPORT_ERROR)),
        }
    }
    None
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
