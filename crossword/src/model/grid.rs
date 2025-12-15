use super::{Coordinate, grid_iterator::GridIterator};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T: Copy> {
    vec: Vec<Vec<T>>,
    size: usize,
}

impl<T: Copy> Grid<T> {
    pub fn from_vec(vec: Vec<Vec<T>>, size: usize) -> Self {
        let grid = Grid { size, vec };
        assert!(grid.validate(), "Grid is not square of the specified size");
        grid
    }

    pub fn new(size: usize, default: T) -> Self {
        let vec = vec![vec![default; size]; size];
        Grid { vec, size }
    }

    pub fn validate(&self) -> bool {
        if self.vec.len() != self.size {
            return false;
        }
        for row in &self.vec {
            if row.len() != self.size {
                return false;
            }
        }
        true
    }

    pub fn vec(&self) -> &Vec<Vec<T>> {
        &self.vec
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, coord: Coordinate) -> Option<T> {
        if !coord.is_valid(self.size, self.size) {
            return None;
        }

        self.vec
            .get(coord.row as usize)
            .and_then(|row| row.get(coord.col as usize))
            .copied()
    }

    pub fn set(&mut self, coord: Coordinate, value: T) {
        self.vec
            .get_mut(coord.row as usize)
            .and_then(|row| row.get_mut(coord.col as usize))
            .map(|cell| *cell = value)
            .expect("Coordinate out of bounds");
    }

    pub fn contains(&self, coord: Coordinate) -> bool {
        coord.is_valid(self.size, self.size)
    }

    // Optional: iterate over all cells with coordinates
    pub fn iter(&self) -> impl Iterator<Item = (Coordinate, T)> {
        self.vec.iter().enumerate().flat_map(|(row, cells)| {
            cells
                .iter()
                .copied()
                .enumerate()
                .map(move |(col, cell)| (Coordinate::new(col as isize, row as isize), cell))
        })
    }

    pub fn coord_iter(&self) -> impl Iterator<Item = Coordinate> {
        (0..self.size).flat_map(move |row| {
            (0..self.size).map(move |col| Coordinate::new(col as isize, row as isize))
        })
    }

    pub fn directional_iter(
        &self,
        start_pos: Coordinate,
        direction: super::MoveDirection,
        should_loop: bool,
    ) -> GridIterator<'_, T> {
        GridIterator::new(self, start_pos, direction, should_loop)
    }
}

#[cfg(test)]
#[path = "grid_tests.rs"]
mod grid_tests;
