use crate::model::{BoardDirection, Coordinate};

pub struct CoordinateIter {
    current: Coordinate,
    end: Coordinate,
    done: bool,
}

impl CoordinateIter {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        CoordinateIter {
            current: start,
            end,
            done: false,
        }
    }

    pub fn direction(&self) -> BoardDirection {
        if self.current.row == self.end.row {
            BoardDirection::Across
        } else {
            BoardDirection::Down
        }
    }
}

impl Iterator for CoordinateIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;

        if self.current == self.end {
            self.done = true;
        } else {
            self.current = match self.direction() {
                BoardDirection::Across => self.current.right(),
                BoardDirection::Down => self.current.down(),
            };
        }

        Some(result)
    }
}

#[cfg(test)]
#[path = "coordinate_iter_tests.rs"]
mod coordinate_iter_tests;
