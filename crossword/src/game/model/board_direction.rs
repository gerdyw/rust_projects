use std::fmt::Display;

use super::MoveDirection;

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

    pub fn to_move_direction(&self) -> MoveDirection {
        match self {
            BoardDirection::Across => MoveDirection::Right,
            BoardDirection::Down => MoveDirection::Down,
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

impl Into<MoveDirection> for BoardDirection {
    fn into(self) -> MoveDirection {
        match self {
            BoardDirection::Across => MoveDirection::Right,
            BoardDirection::Down => MoveDirection::Down,
        }
    }
}

impl Display for BoardDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardDirection::Across => write!(f, "Across"),
            BoardDirection::Down => write!(f, "Down"),
        }
    }
}

#[cfg(test)]
#[path = "board_direction_tests.rs"]
mod board_direction_tests;
