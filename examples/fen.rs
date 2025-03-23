use schachmatt::FEN;

/// Imports the default chess starting position from a fen string
/// and exports it again afterwards
fn main() {
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    println!("{starting_position}");
    let Ok(position) = FEN::import(starting_position) else {
        panic!("Unable to import position");
    };

    let exported_position = FEN::export(&position);
    println!("{exported_position}");
}