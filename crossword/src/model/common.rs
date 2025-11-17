#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BoardDirection {
    #[default]
    Across,
    Down,
}

impl BoardDirection {
    pub fn swap(self) -> Self {
        match self {
            BoardDirection::Across => BoardDirection::Down,
            BoardDirection::Down => BoardDirection::Across,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub col: usize,
    pub row: usize,
    pub size: usize,
}

impl Coordinate {
    pub fn new(col: usize, row: usize, size: usize) -> Self {
        Coordinate { col, row, size }
    }

    pub fn debug(&self) -> String {
        format!(
            "Invalid coordinate: row={}, col={}, size={}",
            self.row, self.col, self.size
        )
    }

    pub fn move_direction(self, direction: MoveDirection) -> Self {
        match direction {
            MoveDirection::Up => self.move_up(),
            MoveDirection::Down => self.move_down(),
            MoveDirection::Left => self.move_left(),
            MoveDirection::Right => self.move_right(),
        }
    }

    pub fn move_right(self) -> Self {
        match self.col + 1 {
            new_x if new_x < self.size => Coordinate::new(new_x, self.row, self.size),
            _ => Coordinate::new(0, self.row, self.size).move_down(),
        }
    }

    pub fn move_down(self) -> Self {
        match self.row + 1 {
            new_y if new_y < self.size => Coordinate::new(self.col, new_y, self.size),
            _ => Coordinate::new(self.col, 0, self.size).move_right(),
        }
    }

    pub fn move_left(self) -> Self {
        match self.col.checked_sub(1) {
            Some(new_x) => Coordinate::new(new_x, self.row, self.size),
            None => Coordinate::new(self.size - 1, self.row, self.size).move_up(),
        }
    }

    pub fn move_up(self) -> Self {
        match self.row.checked_sub(1) {
            Some(new_y) => Coordinate::new(self.col, new_y, self.size),
            None => Coordinate::new(self.col, self.size - 1, self.size).move_left(),
        }
    }
}

#[derive(Clone)]
pub struct Grid<T: Clone>(Vec<Vec<T>>);

impl<T: Clone> Grid<T> {
    pub fn from_vec(vec: Vec<Vec<T>>) -> Self {
        let grid = Grid(vec);
        assert!(
            grid.validate(),
            "Invalid grid: not all rows have the same length"
        );
        grid
    }

    pub fn vec(&self) -> &Vec<Vec<T>> {
        &self.0
    }

    pub fn get(&self, coord: Coordinate) -> &T {
        self.0
            .get(coord.row)
            .and_then(|row| row.get(coord.col))
            .unwrap_or_else(|| panic!("{}", coord.debug()))
    }

    pub fn set(mut self, coord: Coordinate, value: T) -> Self {
        if let Some(row) = self.0.get_mut(coord.row) {
            if let Some(cell) = row.get_mut(coord.col) {
                *cell = value;
            }
        }
        self
    }

    pub fn grid_iter(&'_ self) -> GridTileIterator<'_, T> {
        GridTileIterator {
            grid: &self.0,
            row: 0,
            col: 0,
        }
    }

    pub fn row_iter(&'_ self) -> GridRowIterator<'_, T> {
        GridRowIterator {
            grid: &self.0,
            row: 0,
        }
    }

    fn validate(&self) -> bool {
        let size = self.0.len();
        self.0.iter().all(|row| row.len() == size)
    }
}

pub struct GridTileIterator<'a, T> {
    grid: &'a Vec<Vec<T>>,
    row: usize,
    col: usize,
}

impl<'a, T: Clone> Iterator for GridTileIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.len() {
            return None;
        }

        let current_row = &self.grid[self.row];

        if self.col >= current_row.len() {
            self.row += 1;
            self.col = 0;
            return self.next();
        }

        let item = &current_row[self.col];
        self.col += 1;
        Some(item)
    }
}

pub struct GridRowIterator<'a, T> {
    grid: &'a Vec<Vec<T>>,
    row: usize,
}

impl<'a, T: Clone> Iterator for GridRowIterator<'a, T> {
    type Item = &'a Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.len() {
            return None;
        }
        let item = &self.grid[self.row];
        self.row += 1;
        Some(item)
    }
}

#[cfg(test)]
#[path = "common_tests.rs"]
mod common_tests;
