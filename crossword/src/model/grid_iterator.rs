use crate::model::{Coordinate, Grid, MoveDirection};

pub struct GridIterator<'a, T: Copy> {
    grid: &'a Grid<T>,
    start_pos: Coordinate,
    current_pos: Coordinate,
    direction: MoveDirection,
}

impl<'a, T: Copy> GridIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>, start_pos: Coordinate, direction: MoveDirection) -> Self {
        GridIterator {
            grid,
            start_pos,
            current_pos: start_pos,
            direction,
        }
    }
}

impl<'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = (Coordinate, T);

    // looping through the grid in the specified direction
    // wrapping around edges until it reaches itself again
    fn next(&mut self) -> Option<Self::Item> {
        assert!(
            self.grid.contains(self.start_pos),
            "Start position is out of bounds"
        );

        self.current_pos = self.current_pos.move_direction(self.direction);
        if !self.grid.contains(self.current_pos) {
            // Wrap to the opposite edge and move orthogonally
            match self.direction {
                MoveDirection::Right => {
                    // Hit right edge: wrap col to 0, move row down
                    let new_row = self.current_pos.row + 1;
                    if new_row >= self.grid.size() as isize {
                        // Would go past bottom, wrap to top
                        self.current_pos = Coordinate::new(0, 0);
                    } else {
                        self.current_pos = Coordinate::new(0, new_row);
                    }
                }
                MoveDirection::Down => {
                    // Hit bottom edge: wrap row to 0, move col right
                    let new_col = self.current_pos.col + 1;
                    if new_col >= self.grid.size() as isize {
                        // Would go past right edge, wrap to top-left
                        self.current_pos = Coordinate::new(0, 0);
                    } else {
                        self.current_pos = Coordinate::new(new_col, 0);
                    }
                }
                MoveDirection::Left => {
                    // Hit left edge: wrap col to size-1, move row up
                    let new_row = self.current_pos.row - 1;
                    if new_row < 0 {
                        // Would go past top, wrap to bottom-right
                        self.current_pos = Coordinate::new(
                            (self.grid.size() - 1) as isize,
                            (self.grid.size() - 1) as isize,
                        );
                    } else {
                        self.current_pos = Coordinate::new((self.grid.size() - 1) as isize, new_row);
                    }
                }
                MoveDirection::Up => {
                    // Hit top edge: wrap row to size-1, move col left
                    let new_col = self.current_pos.col - 1;
                    if new_col < 0 {
                        // Would go past left edge, wrap to bottom-right
                        self.current_pos = Coordinate::new(
                            (self.grid.size() - 1) as isize,
                            (self.grid.size() - 1) as isize,
                        );
                    } else {
                        self.current_pos = Coordinate::new(new_col, (self.grid.size() - 1) as isize);
                    }
                }
            }
        }

        if self.current_pos == self.start_pos {
            return None;
        }

        self.grid
            .get(self.current_pos)
            .map(|item| (self.current_pos, item))
    }
}

#[cfg(test)]
#[path = "grid_iterator_tests.rs"]
mod grid_iterator_tests;
