use super::BoardDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MoveDirection {
    pub fn reverse(&self) -> Self {
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

    pub fn into_board_direction(&self) -> BoardDirection {
        match self {
            MoveDirection::Left | MoveDirection::Right => BoardDirection::Across,
            MoveDirection::Up | MoveDirection::Down => BoardDirection::Down,
        }
    }

    pub fn is_backwards(&self) -> bool {
        matches!(self, MoveDirection::Left | MoveDirection::Up)
    }
}

#[cfg(test)]
#[path = "move_direction_tests.rs"]
mod move_direction_tests;
