#[cfg(test)]
mod portable_game_notation_import_tests {
    use schachmatt::PGN;

    #[test]
    fn import_test000_pgn_test() {
        let content = include_str!("./pgn_files/test000.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_test001_pgn_test() {
        let content = include_str!("./pgn_files/test001.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_test002_pgn_test() {
        let content = include_str!("./pgn_files/test002.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_game000_pgn_test() {
        let content = include_str!("./pgn_files/game000.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_game001_pgn_test() {
        let content = include_str!("./pgn_files/game001.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_game002_pgn_test() {
        let content = include_str!("./pgn_files/game002.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_game003_pgn_test() {
        let content = include_str!("./pgn_files/game003.pgn");
        PGN::import(content).unwrap();
    }

    #[test]
    fn import_game004_pgn_test() {
        let content = include_str!("./pgn_files/game004.pgn");
        PGN::import(content).unwrap();
    }
}
