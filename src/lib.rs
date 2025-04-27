#![doc = include_str!("../README.md")]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod constants;
pub use crate::constants::columns as Columns;
pub use crate::constants::fields as Fields;
pub use crate::constants::rows as Rows;

mod game;
pub use crate::game::Game;

mod position;
pub use crate::position::Position;
pub use crate::position::PositionError;

mod field;
pub use crate::field::Field;

mod turn;
pub use crate::turn::Turn;

mod piece;
pub use crate::piece::Piece;

mod piece_type;
pub use crate::piece_type::PieceType;

mod game_result;
pub use crate::game_result::GameResult;

mod player_color;
pub use crate::player_color::PlayerColor;

mod castling_rights;
pub use crate::castling_rights::CastlingRights;

mod ruleset;
pub use crate::ruleset::Ruleset;
pub use crate::ruleset::classic::CLASSIC_RULESET;

mod util;
pub use crate::util::parser::forsyth_edwards_notation as FEN;
pub use crate::util::parser::long_algebraic_notation as LAN;
pub use crate::util::parser::portable_game_notation as PGN;
pub use crate::util::parser::standard_algebraic_notation as SAN;
