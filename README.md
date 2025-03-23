# Schachmatt

Schachmatt is a chess library for rust

## Intention

This library can be used to run chess games, generate legal moves, create and work with standardized chess data formats.

## Examples

The following example starts a game of chess and plays random moves until the game is over

```rust
use schachmatt::{Game, GameResult};
use rand::seq::IndexedRandom;
use rand::rng;

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
```

Examples are available under `examples/`

## Data format support

Schachmatt can import and export chess game, position and moves in the following formats:

- Forsyth-Edwards Notation (FEN)
- Long algebraic notation (LAN)
- Standard algebraic notation (SAN)
- Portable game notation (PGN)
