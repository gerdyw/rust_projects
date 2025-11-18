#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardCell {
    Filled(char),
    Empty,
    Blocked,
}

impl From<Option<char>> for BoardCell {
    fn from(opt: Option<char>) -> Self {
        match opt {
            Some(_) => BoardCell::Empty,
            None => BoardCell::Blocked,
        }
    }
}
