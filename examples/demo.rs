use rand::rng;
use rand::seq::IndexedRandom;
use schachmatt::{Game, GameResult};

/// Starts a game of chess and plays random moves until the game is over.
/// Afterwards the game result is printed.
fn main() {
    let mut game = Game::default();
    let mut rng = rng();

    while game.get_game_result() == GameResult::None {
        let possible_moves = game.get_current_state().get_possible_moves();
        let turn_to_play = possible_moves.choose(&mut rng).unwrap();

        game.execute_turn(*turn_to_play);
    }

    let game_result = match game.get_game_result() {
        GameResult::Draw => "Draw",
        GameResult::Over(player_color) => match player_color {
            schachmatt::PlayerColor::Black => "Black won",
            schachmatt::PlayerColor::White => "White won",
        },
        GameResult::None => unreachable!(),
    };

    println!("{}", game_result);
}
