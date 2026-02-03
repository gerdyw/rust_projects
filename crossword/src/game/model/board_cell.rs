use super::puzzle_cell::PuzzleCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardCell {
    Filled(char),
    Empty,
    Blocked,
}
impl BoardCell {
    pub(crate) fn is_filled(&self) -> bool {
        matches!(self, BoardCell::Filled(_))
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, BoardCell::Empty)
    }

    pub(crate) fn is_blocked(&self) -> bool {
        matches!(self, BoardCell::Blocked)
    }
}

impl From<Option<char>> for BoardCell {
    fn from(opt: Option<char>) -> Self {
        match opt {
            Some(_) => BoardCell::Empty,
            None => BoardCell::Blocked,
        }
    }
}

impl From<PuzzleCell> for BoardCell {
    fn from(cell: PuzzleCell) -> Self {
        match cell {
            PuzzleCell::Fillable(_) => BoardCell::Empty,
            PuzzleCell::Blocked => BoardCell::Blocked,
        }
    }
}

#[cfg(test)]
#[path = "board_cell_tests.rs"]
mod board_cell_tests;
