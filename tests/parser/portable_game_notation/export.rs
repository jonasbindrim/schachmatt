#[cfg(test)]
mod portable_game_notation_export_tests {
    use schachmatt::Pgn;

    #[test]
    fn export_test000_pgn_test() {
        let content = include_str!("./pgn_files/test000.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_test001_pgn_test() {
        let content = include_str!("./pgn_files/test001.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_test002_pgn_test() {
        let content = include_str!("./pgn_files/test002.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_game000_pgn_test() {
        let content = include_str!("./pgn_files/game000.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_game001_pgn_test() {
        let content = include_str!("./pgn_files/game001.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_game002_pgn_test() {
        let content = include_str!("./pgn_files/game002.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_game003_pgn_test() {
        let content = include_str!("./pgn_files/game003.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }

    #[test]
    fn export_game004_pgn_test() {
        let content = include_str!("./pgn_files/game004.pgn");
        let mut game = Pgn::import(content).unwrap();
        let game_export = Pgn::export(&mut game);

        let game_import = Pgn::import(&game_export).unwrap();
        assert_eq!(game_import.get_current_state(), game.get_current_state());
    }
}
