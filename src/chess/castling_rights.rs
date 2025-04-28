#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CastlingRights {
    queenside: bool,
    kingside: bool,
}

impl CastlingRights {
    pub fn new(queenside: bool, kingside: bool) -> Self {
        CastlingRights {
            queenside,
            kingside,
        }
    }

    pub fn set_queenside(&mut self, queenside: bool) {
        self.queenside = queenside
    }

    pub fn set_kingside(&mut self, kingside: bool) {
        self.kingside = kingside
    }

    pub fn get_queenside(&self) -> bool {
        self.queenside
    }

    pub fn get_kingside(&self) -> bool {
        self.kingside
    }
}
