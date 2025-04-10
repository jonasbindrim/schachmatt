use rand::rng;
use rand::seq::IndexedRandom;
use schachmatt::{Game, GameResult, PlayerColor};

/// Starts a game of chess and plays random moves until the game is over.
/// Afterwards the game result is printed.
fn main() {
    let mut game = Game::default();
    let mut rng = rng();

    while game.get_game_result().is_none() {
        let possible_moves = game.get_current_state().get_possible_moves();
        let turn_to_play = possible_moves.choose(&mut rng).unwrap();

        game.execute_turn(*turn_to_play).unwrap();
    }

    let game_result = match game.get_game_result().unwrap() {
        GameResult::Draw => "Draw",
        GameResult::Over(player_color) => match player_color {
            PlayerColor::Black => "Black won",
            PlayerColor::White => "White won",
        },
    };

    println!("{}", game_result);
}
