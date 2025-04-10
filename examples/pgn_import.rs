use schachmatt::PGN;

static PGN_CONTENT: &str = "[Event \"New Orleans\"]
[Site \"New Orleans\"]
[Date \"1848.??.??\"]
[Round \"?\"]
[White \"Morphy, Paul \"]
[Black \"Morphy, Alonzo\"]
[Result \"1-0\"]
[WhiteElo \"\"]
[BlackElo \"\"]
[ECO \"C23\"]

1.e4 e5 2.Bc4 f5 3.exf5 Nf6 4.Nc3 d5 5.Nxd5 Bc5 6.Nxf6+ Qxf6 7.d3 Bxf5 8.Nf3 Bg4
9.Bd5 c6 10.Be4 Nd7 11.O-O h6 12.c3 O-O-O 13.b4 Bb6 14.a4 a6 15.Qb3 Bxf3
16.Bxf3 g5 17.Be3 g4 18.Bxg4 Bc7 19.Bf3 Rhg8 20.Be4 Rg4 21.f3 Rg7 22.b5 axb5
23.axb5 Nb6 24.bxc6 Rdg8 25.Rf2 Qd8 26.Ra8+ Bb8 27.Bxb6 Rxg2+ 28.Rxg2 Rxg2+
29.Kxg2 Qg5+ 30.Kh1 Qc1+ 31.Bg1  1-0";

/// Imports a pgn file into a game
fn main() {
    let Ok(game) = PGN::import(PGN_CONTENT) else {
        panic!("Unable to import pgn content");
    };

    println!("{}", PGN::export(&game));
}
