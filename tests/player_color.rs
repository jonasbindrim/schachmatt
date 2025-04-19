#[cfg(test)]
mod player_color {
    use schachmatt::PlayerColor;

    #[test]
    fn test_reverse_black() {
        let black = PlayerColor::Black;
        assert_eq!(black.reverse(), PlayerColor::White);
    }

    #[test]
    fn test_reverse_white() {
        let white = PlayerColor::White;
        assert_eq!(white.reverse(), PlayerColor::Black);
    }
}
