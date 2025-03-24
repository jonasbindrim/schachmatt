use rand::rng;
use rand::seq::IndexedRandom;
use schachmatt::{Game, PGN};

/// Starts a game of chess and plays random moves until the game is over.
/// Afterwards the game result is printed.
fn main() {
    let mut game = random_game();
    let pgn = PGN::export(&mut game);
    println!("{}", pgn);
}

fn random_game() -> Game {
    let mut game = Game::default();
    let mut rng = rng();

    while game.get_game_result().is_none() {
        let possible_moves = game.get_current_state().get_possible_moves();
        let turn_to_play = possible_moves.choose(&mut rng).unwrap();

        game.execute_turn(*turn_to_play);
    }

    game
}