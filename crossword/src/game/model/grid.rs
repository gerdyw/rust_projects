use super::{Coordinate, grid_iterator::GridIterator};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T: Copy> {
    vec: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    pub fn from_vec(vec: Vec<Vec<T>>, width: usize, height: usize) -> Self {
        let grid = Grid { width, height, vec };
        assert!(grid.validate(), "Grid dimensions do not match the data");
        grid
    }

    pub fn new(width: usize, height: usize, default: T) -> Self {
        let vec = vec![vec![default; width]; height];
        Grid { vec, width, height }
    }

    pub fn validate(&self) -> bool {
        if self.vec.len() != self.height {
            return false;
        }
        for row in &self.vec {
            if row.len() != self.width {
                return false;
            }
        }
        true
    }

    pub fn vec(&self) -> &Vec<Vec<T>> {
        &self.vec
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    // Deprecated: use width() or height() instead
    pub fn size(&self) -> usize {
        // For backward compatibility with square grids, return width
        // (assumes width == height for existing code)
        self.width
    }

    pub fn get(&self, coord: Coordinate) -> Option<T> {
        if !coord.is_valid(self.width, self.height) {
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
        coord.is_valid(self.width, self.height)
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
        let width = self.width;
        (0..self.height).flat_map(move |row| {
            (0..width).map(move |col| Coordinate::new(col as isize, row as isize))
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
