use crate::model::{BoardDirection, MoveDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub col: isize,
    pub row: isize,
}

impl Coordinate {
    pub fn new(col: isize, row: isize) -> Self {
        Coordinate { col, row }
    }

    pub fn set_row(&self, row: isize) -> Self {
        Coordinate { col: self.col, row }
    }

    pub fn set_col(&self, col: isize) -> Self {
        Coordinate { col, row: self.row }
    }

    pub fn set_axis(&self, direction: BoardDirection, value: isize) -> Self {
        match direction {
            BoardDirection::Across => self.set_col(value),
            BoardDirection::Down => self.set_row(value),
        }
    }

    // Basic movements - just return new coordinate, no bounds checking
    pub fn up(&self) -> Self {
        Coordinate::new(self.col, self.row - 1)
    }

    pub fn down(&self) -> Self {
        Coordinate::new(self.col, self.row + 1)
    }

    pub fn left(&self) -> Self {
        Coordinate::new(self.col - 1, self.row)
    }

    pub fn right(&self) -> Self {
        Coordinate::new(self.col + 1, self.row)
    }

    // Boundary checking
    pub fn is_valid(&self, max_col: usize, max_row: usize) -> bool {
        self.col >= 0 && self.col < max_col as isize && self.row >= 0 && self.row < max_row as isize
    }

    pub fn move_direction(&self, direction: MoveDirection) -> Self {
        match direction {
            MoveDirection::Right => self.right(),
            MoveDirection::Down => self.down(),
            MoveDirection::Left => self.left(),
            MoveDirection::Up => self.up(),
        }
    }
}

#[cfg(test)]
#[path = "coordinate_tests.rs"]
mod coordinate_tests;
