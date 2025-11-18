use crate::model::MoveDirection;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BoardDirection {
    #[default]
    Across,
    Down,
}
impl BoardDirection {
    pub fn swap(&self) -> Self {
        match self {
            BoardDirection::Across => BoardDirection::Down,
            BoardDirection::Down => BoardDirection::Across,
        }
    }
}

impl From<MoveDirection> for BoardDirection {
    fn from(direction: MoveDirection) -> Self {
        match direction {
            MoveDirection::Left | MoveDirection::Right => BoardDirection::Across,
            MoveDirection::Up | MoveDirection::Down => BoardDirection::Down,
        }
    }
}
