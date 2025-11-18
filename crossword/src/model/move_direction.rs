#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MoveDirection {
    pub fn opposite(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Up,
        }
    }

    pub fn orthogonal(&self) -> MoveDirection {
        match self {
            MoveDirection::Left => MoveDirection::Up,
            MoveDirection::Right => MoveDirection::Down,
            MoveDirection::Up => MoveDirection::Left,
            MoveDirection::Down => MoveDirection::Right,
        }
    }
}
