#[cfg(test)]
mod tests {
    use crate::{FEN, PGN};

    #[test]
    fn pgn_import_test() {
        for pgn in TESTS {
            let game = PGN::import(pgn).unwrap();
            assert_eq!(
                FEN::export(&game.get_current_state()),
                game.get_metadata("FEN").unwrap()
            );
        }
    }

    #[test]
    fn pgn_export_test() {
        for pgn in TESTS {
            let mut game = PGN::import(pgn).unwrap();
            let game_export = PGN::export(&mut game);

            let game_import = PGN::import(&game_export).unwrap();
            assert_eq!(game_import.get_current_state(), game.get_current_state());
        }
    }

    pub static PGNTEST1: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2023.01.21"]
[Round "-"]
[White "NyxFox"]
[Black "mayurwadhwani"]
[Result "1-0"]
[CurrentPosition "r7/6kp/1p1pBQp1/1PpP2P1/p1P5/5P2/1P4K1/4Rn2 b - -"]
[Timezone "UTC"]
[ECO "B00"]
[ECOUrl "https://www.chess.com/openings/Kings-Pawn-Opening-Owens-Defense-2.d4-Bb7-3.f3"]
[UTCDate "2023.01.21"]
[UTCTime "14:48:50"]
[WhiteElo "1301"]
[BlackElo "1301"]
[TimeControl "300"]
[Termination "NyxFox won by checkmate"]
[StartTime "14:48:50"]
[EndDate "2023.01.21"]
[EndTime "14:55:25"]
[Link "https://www.chess.com/game/live/68051594937"]
[FEN "r7/6kp/1p1pBQp1/1PpP2P1/p1P5/5P2/1P4K1/4Rn2 b - - 0 33"]

1. e4 b6 2. d4 Bb7 3. f3 e6 4. c4 Nf6 5. Nh3 d6 6. Qb3 Be7 7. Be2 Nbd7 $6 8. Be3
O-O 9. Nc3 a5 10. a3 c5 11. d5 e5 $2 12. Nb5 Ba6 13. a4 Bxb5 14. axb5 Nh5 15. O-O
f5 $6 16. exf5 Nf4 $2 17. Nxf4 $6 exf4 18. Bxf4 Rxf5 19. Bg3 Rg5 $6 20. Rae1 a4 21.
Qc2 Bf6 $6 22. Bh4 $2 Bd4+ $2 23. Kh1 Bf6 24. Bxg5 Bxg5 25. Bd3 Nf6 $6 26. Bf5 $6 Qf8 $6
27. g3 Nh5 28. h4 $4 g6 $4 29. Be6+ $1 Kg7 $6 30. hxg5 Nxg3+ 31. Kg2 Nxf1 $2 32. Qc3+
Qf6 33. Qxf6# 1-0"#;

    pub static PGNTEST2: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2023.01.20"]
[Round "-"]
[White "124elcapo1"]
[Black "NyxFox"]
[Result "1-0"]
[CurrentPosition "1k6/8/4R3/6r1/1P3R2/P4B2/7p/7K b - -"]
[Timezone "UTC"]
[ECO "C23"]
[ECOUrl "https://www.chess.com/openings/Bishops-Opening"]
[UTCDate "2023.01.20"]
[UTCTime "16:37:22"]
[WhiteElo "1340"]
[BlackElo "1285"]
[TimeControl "300"]
[Termination "124elcapo1 won by resignation"]
[StartTime "16:37:22"]
[EndDate "2023.01.20"]
[EndTime "16:45:17"]
[Link "https://www.chess.com/game/live/67971763847"]
[FEN "1k6/8/4R3/6r1/1P3R2/P4B2/7p/7K b - - 0 48"]

1. e4 e5 2. Bc4 d6 3. a3 a6 4. d3 b5 5. Bb3 c5 6. Nc3 Nf6 7. Bg5 Be7 8. h3 h6 9.
Bh4 g5 10. Bg3 Bb7 $6 11. Nf3 Nbd7 12. Qe2 Qc7 13. Bh2 Rf8 $6 14. g4 $6 O-O-O 15.
Qd2 $4 Nb6 $4 16. O-O $4 h5 17. Nxg5 hxg4 $2 18. Nxf7 $2 gxh3 $4 19. Nxd8 Bxd8 20. Qh6 $6
Rg8+ $4 21. Kh1 Qg7 $4 22. Qxg7 Rxg7 23. f3 $2 c4 24. dxc4 bxc4 25. Ba2 d5 26.
exd5 $6 Nfxd5 $2 27. Bxe5 $6 Rg6 $2 28. Ne4 Ne3 29. Rfc1 $4 Rh6 $4 30. Bf4 $1 Nxc2 31.
Rxc2 Rh4 32. Be3 $6 Bxe4 33. fxe4 Rxe4 34. Bxb6 Bxb6 35. Bxc4 Kb8 36. Bxa6 Bc7
37. Bd3 Rh4 $6 38. Rac1 Bb6 $6 39. b4 $6 Kb7 40. Bf5 $6 Be3 $2 41. Re1 Bf4 $2 42. Re7+
Kb6 43. Re6+ Kb7 $6 44. Rcc6 $2 Rh5 45. Bg4 $2 Rg5 $2 46. Bf3 h2 $6 47. Rc4+ Kb8 48.
Rxf4 {<br /><br />So hätte die Partien weitergehen können...} (48. Rxf4 Rg1+ 49.
Kxh2 Rg7 50. Ra6 Rh7+ 51. Kg3 Rg7+ 52. Kf2 {+66.8}) 1-0"#;

    pub static PGNTEST3: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2023.01.07"]
[Round "?"]
[White "MDSailor"]
[Black "WalkingWard"]
[Result "1-0"]
[ECO "B12"]
[WhiteElo "1284"]
[BlackElo "1256"]
[TimeControl "300"]
[EndTime "11:13:16 PST"]
[Termination "MDSailor won by resignation"]
[FEN "8/2k3pp/8/1pR1P3/1P1r4/P7/5b1P/5K2 b - - 1 37"]

1. e4 c6 2. d4 d5 3. e5 c5 4. c3 Nc6 5. Bb5 a6 6. Bxc6+ bxc6 7. Nf3 Bg4 8. Qa4
Qb6 9. O-O Bxf3 10. gxf3 cxd4 11. cxd4 e6 12. b3 Ne7 13. Be3 Nf5 14. Rd1 Be7 15.
Nc3 O-O 16. Rab1 Nh4 17. f4 f6 18. a3 fxe5 19. fxe5 Nf5 20. b4 Bh4 21. Qc2 Nxe3
22. fxe3 Rf2 23. Qd3 Raf8 24. Na4 Qb5 25. Qxb5 cxb5 26. Nc5 R8f3 27. Nxe6 Rxe3
28. Rbc1 Ree2 29. Rc8+ Kf7 30. Rf1 Kxe6 31. Rxf2 Rxf2 32. Rc6+ Kd7 33. Rxa6 Rd2
34. Rd6+ Kc7 35. Rxd5 Bf2+ 36. Kf1 Rxd4 37. Rc5+ 1-0"#;

    pub static PGNTEST4: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2023.01.07"]
[Round "-"]
[White "jayfresh87"]
[Black "WalkingWard"]
[Result "0-1"]
[CurrentPosition "8/8/k3b3/P1Rp1p2/4r3/3K4/8/8 b - -"]
[Timezone "UTC"]
[ECO "B12"]
[ECOUrl "https://www.chess.com/openings/Caro-Kann-Defense-2.d4"]
[UTCDate "2023.01.07"]
[UTCTime "18:51:05"]
[WhiteElo "1286"]
[BlackElo "1272"]
[TimeControl "300"]
[Termination "WalkingWard won by resignation"]
[StartTime "18:51:05"]
[EndDate "2023.01.07"]
[EndTime "18:58:10"]
[Link "https://www.chess.com/game/live/66856392603"]
[FEN "8/8/k3b3/P1Rp1p2/4r3/3K4/8/8 b - - 0 51"]

1. d4 c6 2. e4 d5 3. e5 c5 4. c3 Nc6 5. dxc5 e6 6. b4 $6 a5 7. b5 $2 Na7 $4 8. b6 $4
Nc6 9. Be3 Nxe5 10. f4 $2 Nc6 $2 11. a4 Nge7 $6 12. g4 $2 h5 $6 13. gxh5 $4 Nf5 $1 14. Bf2
Qf6 $2 15. Qg4 $2 Nh6 $4 16. Qg5 $2 Be7 $2 17. h4 Nf5 18. Nf3 g6 $2 19. hxg6 fxg6 $2 20. h5 $4
Qg7 21. Qxg6+ $4 Qxg6 22. hxg6 Rxh1 23. Ke2 Kf8 24. Nbd2 Kg7 25. Rd1 Kxg6 26.
Ne5+ $6 Nxe5 27. fxe5 Rh2 28. Nf3 Rxf2+ 29. Kxf2 Bxc5+ 30. Nd4 Bxb6 31. Kf3 Bxd4
32. cxd4 b6 33. Bd3 Ba6 34. Rg1+ Kf7 35. Bxf5 exf5 36. Rg5 Ke6 $2 37. Rg6+ Kf7 38.
Rxb6 Bc8 39. Rd6 Be6 40. Rb6 $6 Ke7 41. Rb7+ Kd8 $6 42. Kf4 Kc8 43. Rh7 Rb8 $6 44.
Ra7 Rb4 45. Rxa5 $6 Rxd4+ 46. Ke3 $6 Re4+ 47. Kd3 Rxe5 48. Rc5+ Kb7 49. Rb5+ Ka6
50. Rc5 $6 Re4 51. a5 {<br /><br />So hätte die Partien weitergehen können...}
(51. a5 Rc4 52. Rxc4 dxc4+ 53. Ke3 Kxa5 54. Kd4 Kb4 55. Ke3 {-60.6}) 0-1"#;

    pub static PGNTEST5: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2023.01.13"]
[Round "-"]
[White "lmontz"]
[Black "WalkingWard"]
[Result "1-0"]
[CurrentPosition "B5r1/p1b2p2/n3p1pk/2q3Np/5P1n/P3Q1R1/1P3P1P/6RK b - -"]
[Timezone "UTC"]
[ECO "B10"]
[ECOUrl "https://www.chess.com/openings/Caro-Kann-Defense-Hillbilly-Attack-2...d5"]
[UTCDate "2023.01.13"]
[UTCTime "15:38:57"]
[WhiteElo "1264"]
[BlackElo "1217"]
[TimeControl "180"]
[Termination "lmontz won by resignation"]
[StartTime "15:38:57"]
[EndDate "2023.01.13"]
[EndTime "15:43:54"]
[Link "https://www.chess.com/game/live/67363390229"]
[FEN "B5r1/p1b2p2/n3p1pk/2q3Np/5P1n/P3Q1R1/1P3P1P/6RK b - - 0 36"]

1. e4 c6 2. Bc4 d5 3. exd5 cxd5 4. Bb3 Nc6 5. d4 Bf5 6. Nf3 e6 7. O-O Bd6 8. c3
Nge7 9. c4 O-O 10. c5 Bc7 11. Nc3 Bg4 12. Bg5 Qd7 13. Qd3 h6 14. Bd2 Nf5 15. Be3
Nb4 16. Qd2 Bxf3 17. gxf3 Nh4 18. Qe2 Kh7 19. Kh1 b6 20. Rg1 bxc5 21. dxc5 Rg8
22. a3 Na6 23. Bc2+ g6 24. Rg4 Nf5 25. Rag1 h5 26. R4g2 Nh4 27. Rg3 Nf5 28. Rg5
Bd8 29. R5g2 Nh4 30. Rg3 Bc7 31. f4 d4 32. Ne4 dxe3 33. Ng5+ Kh6 34. Qxe3 Qc6+
35. Be4 Qxc5 36. Bxa8 1-0"#;

    pub static TESTS: [&str; 5] = [PGNTEST1, PGNTEST2, PGNTEST3, PGNTEST4, PGNTEST5];
}
