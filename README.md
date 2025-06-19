# Schachmatt

[![Crates.io](https://img.shields.io/crates/v/schachmatt?style=flat-square)](https://crates.io/crates/schachmatt)
[![Crates.io](https://img.shields.io/crates/d/schachmatt?style=flat-square)](https://crates.io/crates/schachmatt)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

Schachmatt is a chess library written in rust. This library can be used to run chess games, generate legal moves or create and work with standardized chess data formats.

This crate is not written with speed in mind. The goal is not to provide the fastes chess library in rust. Therefore I would strongly advise to not write a chess engine using this library.

If you find any issues or have suggestions for new features feel free to open an issue or open a pull request.

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
        let possible_moves = game.get_possible_turns();
        let turn_to_play = possible_moves.choose(&mut rng).unwrap();

        game.execute_turn(*turn_to_play);
    }

    let game_result = match game.get_game_result().unwrap() {
        GameResult::Draw => "Draw",
        GameResult::Decisive(player_color) => match player_color {
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

## Rulesets

Rulesets can be used to change behaviour of a game of chess. Currently, only the classical chess ruleset is implemented.

User can implement their own rulesets by using the `Ruleset`-module.

## Data format support

Schachmatt support multiple data interchange formats standardized for usage in chess.
Below you find the currently supported format and if you follow the links and explanaition
of the different formats. Schachmatt can import and export data in the following data formats:

- Forsyth-Edwards Notation (FEN): Import and export single positions
  - `https://www.chessprogramming.org/Forsyth-Edwards_Notation`
- Long algebraic notation (LAN): Import and export single moves
  - `https://www.chessprogramming.org/Algebraic_Chess_Notation#Long_Algebraic_Notation_.28LAN.29`
- Standard algebraic notation (SAN): Import and export single moves
  - `https://www.chessprogramming.org/Algebraic_Chess_Notation#Standard_Algebraic_Notation_.28SAN.29`
- Portable game notation (PGN): Import and export whole games
  - `https://www.chessprogramming.org/Portable_Game_Notation`
