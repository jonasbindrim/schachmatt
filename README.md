# Schachmatt

Schachmatt is a chess library for rust.

[![Crates.io](https://img.shields.io/crates/v/schachmatt?style=flat-square)](https://crates.io/crates/schachmatt)
[![Crates.io](https://img.shields.io/crates/d/schachmatt?style=flat-square)](https://crates.io/crates/schachmatt)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

## Intention

This library can be used to run chess games, generate legal moves or create and work with standardized chess data formats.

## Examples

The following example starts a game of chess and plays random moves until the game is over

```rust
use schachmatt::{Game, GameResult, PlayerColor};
use rand::seq::IndexedRandom;
use rand::rng;

/// Starts a game of chess and plays random moves until the game is over.
/// Afterwards the game result is printed.
fn main() {

    let mut game = Game::default();
    let mut rng = rng();

    while game.get_game_result().is_none() {
        let possible_moves = game.get_current_state().get_possible_moves();
        let turn_to_play = possible_moves.choose(&mut rng).unwrap();

        game.execute_turn(*turn_to_play);
    }

    let game_result = match game.get_game_result().unwrap() {
        GameResult::Draw => "Draw",
        GameResult::Over(player_color) => match player_color {
            PlayerColor::Black => "Black won",
            PlayerColor::White => "White won",
        }
    };

    println!("{}", game_result);
}
```

See more examples:

- [examples](examples/)
- [doc.rs](https://docs.rs/schachmatt/latest/schachmatt/)

## Data format support

Schachmatt can import and export chess games, position and moves in the following formats:

- Forsyth-Edwards Notation (FEN)
- Long algebraic notation (LAN)
- Standard algebraic notation (SAN)
- Portable game notation (PGN)
