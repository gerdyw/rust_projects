use super::{Coordinate, Grid, MoveDirection};

#[derive(Debug)]
pub struct GridIterator<'a, T: Copy> {
    grid: &'a Grid<T>,
    start_pos: Coordinate,
    current_pos: Coordinate,
    direction: MoveDirection,
    should_loop: bool,
    has_started: bool,
}

impl<'a, T: Copy> GridIterator<'a, T> {
    pub fn new(
        grid: &'a Grid<T>,
        start_pos: Coordinate,
        direction: MoveDirection,
        should_loop: bool,
    ) -> Self {
        assert!(grid.contains(start_pos), "Start position is out of bounds");
        GridIterator {
            grid,
            start_pos,
            current_pos: start_pos,
            direction,
            should_loop,
            has_started: false,
        }
    }
}

impl<'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = (Coordinate, T);

    // looping through the grid in the specified direction
    // wrapping around edges until it reaches itself again
    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.contains(self.current_pos) {
            return None;
        }

        // If we've looped back to start after having started, we're done
        if self.has_started && self.current_pos == self.start_pos {
            return None;
        }

        self.has_started = true;

        // Get current position and value
        let result = self
            .grid
            .get(self.current_pos)
            .map(|value| (self.current_pos, value));

        // Move to next position
        self.current_pos = self.current_pos.move_direction(self.direction);

        if !self.grid.contains(self.current_pos) && self.should_loop {
            // Wrap around
            match self.direction {
                MoveDirection::Right => {
                    self.current_pos = Coordinate::new(0, self.current_pos.row);
                }
                MoveDirection::Down => {
                    self.current_pos = Coordinate::new(self.current_pos.col, 0);
                }
                MoveDirection::Left => {
                    self.current_pos =
                        Coordinate::new((self.grid.width() - 1) as isize, self.current_pos.row);
                }
                MoveDirection::Up => {
                    self.current_pos =
                        Coordinate::new(self.current_pos.col, (self.grid.height() - 1) as isize);
                }
            }
        }

        result
    }
}

#[cfg(test)]
#[path = "grid_iterator_tests.rs"]
mod grid_iterator_tests;
