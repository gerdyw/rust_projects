use crate::model::{BoardCell, BoardDirection, Coordinate, Grid, MoveDirection, Puzzle};

pub struct PlayerBoard {
    puzzle: Puzzle,
    size: usize,
    grid: Grid<BoardCell>,
    empty_cells: usize,
    cursor: Coordinate,
    direction: BoardDirection,
}

impl PlayerBoard {
    pub fn from_puzzle(puzzle: &Puzzle) -> Self {
        let size = puzzle.size();

        let grid = puzzle
            .grid
            .vec()
            .iter()
            .map(|row| row.iter().map(|&cell| cell.into()).collect())
            .collect();

        let empty_cells = puzzle
            .grid
            .iter()
            .filter(|&(_, &cell)| cell.is_some())
            .count();

        PlayerBoard {
            puzzle: puzzle.clone(),
            size,
            grid: Grid::from_vec(grid, size),
            cursor: Coordinate::new(0, 0),
            direction: BoardDirection::Across,
            empty_cells,
        }
    }

    pub fn swap_direction(self) -> Self {
        let new_direction = match self.direction {
            BoardDirection::Across => BoardDirection::Down,
            BoardDirection::Down => BoardDirection::Across,
        };

        PlayerBoard {
            direction: new_direction,
            ..self
        }
    }

    pub fn move_cursor(self, move_direction: MoveDirection) -> Self {
        let mut cursor = self.cursor.move_direction(move_direction);
        let mut iterations = 0;
        let max_iterations = (self.size * self.size) as usize;

        while !self.is_playable(cursor) && iterations < max_iterations {
            if !self.grid.contains(cursor) {
                cursor = cursor
                    .move_direction(move_direction.orthogonal())
                    .set_axis(move_direction.into(), 0);
            } else {
                cursor = cursor.move_direction(move_direction);
            }
            iterations += 1;
        }

        PlayerBoard { cursor, ..self }
    }

    pub fn is_playable(&self, coord: Coordinate) -> bool {
        self.puzzle.is_fillable(coord)
    }
}
