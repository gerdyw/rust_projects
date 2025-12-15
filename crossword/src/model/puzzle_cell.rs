#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PuzzleCell {
    Blocked,
    Fillable(char),
}

impl PuzzleCell {
    pub fn is_fillable(&self) -> bool {
        matches!(self, PuzzleCell::Fillable(_))
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, PuzzleCell::Blocked)
    }

    pub fn get_char(&self) -> Option<char> {
        match self {
            PuzzleCell::Fillable(c) => Some(*c),
            PuzzleCell::Blocked => None,
        }
    }
}
