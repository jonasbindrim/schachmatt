#[cfg(test)]
mod portable_game_notation_import_tests {
    use schachmatt::{GameResult, Pgn, PlayerColor};

    #[test]
    fn import_test000_pgn_test() {
        let content = include_str!("./pgn_files/test000.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_test001_pgn_test() {
        let content = include_str!("./pgn_files/test001.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_test002_pgn_test() {
        let content = include_str!("./pgn_files/test002.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_test003_pgn_test() {
        let content = include_str!("./pgn_files/test003.pgn");
        let game = Pgn::import(content).unwrap();
        assert_eq!(
            game.get_game_result(),
            Some(GameResult::Decisive(PlayerColor::White))
        );
    }

    #[test]
    fn import_game000_pgn_test() {
        let content = include_str!("./pgn_files/game000.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_game001_pgn_test() {
        let content = include_str!("./pgn_files/game001.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_game002_pgn_test() {
        let content = include_str!("./pgn_files/game002.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_game003_pgn_test() {
        let content = include_str!("./pgn_files/game003.pgn");
        Pgn::import(content).unwrap();
    }

    #[test]
    fn import_game004_pgn_test() {
        let content = include_str!("./pgn_files/game004.pgn");
        Pgn::import(content).unwrap();
    }
}
