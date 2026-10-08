use std::{cmp::Ordering, fmt::Display};

use super::{BoardDirection, MoveDirection};

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

    pub fn add_to_axis(&self, direction: BoardDirection, delta: isize) -> Self {
        match direction {
            BoardDirection::Across => self.set_col(self.col + delta),
            BoardDirection::Down => self.set_row(self.row + delta),
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

    pub fn order_board_dir(&self, other: &Coordinate, direction: BoardDirection) -> Ordering {
        // compare self and other based on the specified board direction
        // using row and col values
        let orth_compare = match direction {
            BoardDirection::Across => self.row.cmp(&other.row),
            BoardDirection::Down => self.col.cmp(&other.col),
        };

        if orth_compare != Ordering::Equal {
            return orth_compare;
        }

        match direction {
            BoardDirection::Across => self.col.cmp(&other.col),
            BoardDirection::Down => self.row.cmp(&other.row),
        }
    }

    pub fn order_in_move_dir(&self, other: &Coordinate, direction: MoveDirection) -> Ordering {
        let board_dir_compare = self.order_board_dir(other, direction.into_board_direction());
        if direction.is_backwards() {
            board_dir_compare.reverse()
        } else {
            board_dir_compare
        }
    }

    pub fn on_same_line(&self, other: Coordinate, direction: BoardDirection) -> bool {
        match direction {
            BoardDirection::Across => self.row == other.row,
            BoardDirection::Down => self.col == other.col,
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.col),
            other => other,
        }
    }
}

#[cfg(test)]
#[path = "coordinate_tests.rs"]
mod coordinate_tests;
