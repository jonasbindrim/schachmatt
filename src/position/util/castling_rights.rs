#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct CastlingRights {
    pub queenside: bool,
    pub kingside: bool,
}
