use crate::model::{
    BoardCell, BoardDirection, Coordinate, Grid, MoveDirection, Puzzle, PuzzleCell, Word,
};

pub struct PlayerBoard {
    puzzle: Puzzle,
    size: usize,
    grid: Grid<BoardCell>,
    cursor: Coordinate,
    direction: BoardDirection,
    empty_cells: usize,
}

impl PlayerBoard {
    pub fn from_puzzle(puzzle: Puzzle) -> Self {
        let size = puzzle.size();

        let grid = puzzle
            .grid
            .vec()
            .iter()
            .map(|row| row.iter().map(|&cell| cell.into()).collect())
            .collect();

        let empty_cells = puzzle
            .grid
            .vec()
            .iter()
            .flatten()
            .filter(|&&cell| matches!(cell, PuzzleCell::Fillable(_)))
            .count();

        let mut board = PlayerBoard {
            puzzle: puzzle,
            size,
            grid: Grid::from_vec(grid, size),
            cursor: Coordinate::new(0, 0),
            direction: BoardDirection::Across,
            empty_cells,
        };

        board.move_to_next_empty_cell();
        board
    }

    pub fn get(&self, coord: Coordinate) -> BoardCell {
        self.grid.get(coord).unwrap_or(BoardCell::Blocked)
    }

    pub fn get_clue_number_at(&self, coord: Coordinate) -> Option<usize> {
        self.puzzle.get_clue_number_at(coord)
    }

    pub fn get_current(&self) -> BoardCell {
        self.get(self.cursor)
    }

    pub fn get_cursor(&self) -> Coordinate {
        self.cursor
    }

    pub fn get_direction(&self) -> BoardDirection {
        self.direction
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_words_in_current_direction(&self) -> &Vec<Word> {
        match self.direction {
            BoardDirection::Across => &self.puzzle.words.across,
            BoardDirection::Down => &self.puzzle.words.down,
        }
    }

    pub fn get_current_word(&self) -> Option<&Word> {
        self.puzzle.get_word_at(self.cursor, self.direction)
    }

    pub fn swap_direction(&mut self) {
        self.direction = self.direction.swap();
    }

    pub fn move_cursor(&mut self, move_direction: MoveDirection) {
        let new_pos = self
            .grid
            .directional_iter(self.cursor, move_direction)
            .find(|(_, cell)| *cell != BoardCell::Blocked)
            .map(|(coord, _)| coord)
            .unwrap_or(self.cursor);

        self.cursor = new_pos;
    }

    pub fn move_forward(&mut self) {
        let move_direction = self.direction.into();
        self.move_cursor(move_direction);
    }

    pub fn move_backward(&mut self) {
        let move_direction = self.direction.to_move_direction().reverse();
        self.move_cursor(move_direction);
    }

    pub fn move_to_next_empty_cell(&mut self) {
        let move_direction = self.direction.into();
        self.move_to_cell_that(|cell| cell == BoardCell::Empty, move_direction);
    }

    pub fn move_to_previous_empty_cell(&mut self) {
        let move_direction = self.direction.to_move_direction().reverse();
        self.move_to_cell_that(|cell| cell == BoardCell::Empty, move_direction);
    }

    pub fn move_to_next_open_word(&mut self) {
        let current_clue_number = self.get_current_word().map(|w| w.clue_number);
        let starting_pos = self.cursor;

        while self.get_current_word().is_none()
            || self.get_current_word().map(|w| w.clue_number) == current_clue_number
        {
            self.move_to_next_empty_cell();

            if &self.cursor <= &starting_pos {
                self.swap_direction();
            }

            if self.cursor == starting_pos {
                break;
            }
        }
    }

    pub fn write_to_cell(&mut self, c: char) {
        assert!(
            c.is_alphanumeric(),
            "Only alphanumeric characters can be entered"
        );
        assert!(
            self.is_playable(self.cursor),
            "Cannot fill a blocked cell at {:?}",
            self.cursor
        );

        let current_cell = self.get(self.cursor);

        self.grid.set(self.cursor, BoardCell::Filled(c));
        if current_cell == BoardCell::Empty {
            self.empty_cells = self.empty_cells.saturating_sub(1);
        }
    }

    pub fn clear_cell(&mut self) {
        let current_cell = self.get(self.cursor);
        self.grid.set(self.cursor, BoardCell::Empty);
        if current_cell != BoardCell::Empty {
            self.empty_cells = self.empty_cells.saturating_add(1);
        }
    }

    pub fn is_playable(&self, coord: Coordinate) -> bool {
        self.get(coord) != BoardCell::Blocked
    }

    pub fn has_won(&self) -> bool {
        self.grid.coord_iter().all(|coord| self.cell_correct(coord))
    }

    fn move_to_cell_that<F>(&mut self, condition: F, move_direction: MoveDirection)
    where
        F: Fn(BoardCell) -> bool,
    {
        let new_pos = self
            .grid
            .directional_iter(self.cursor, move_direction)
            .find(|(_, cell)| condition(*cell))
            .map(|(coord, _)| coord)
            .unwrap_or(self.cursor);

        self.cursor = new_pos;
    }

    fn cell_correct(&self, coord: Coordinate) -> bool {
        let player_cell = self.get(coord);
        let puzzle_cell = self.puzzle.get(coord);

        match (player_cell, puzzle_cell) {
            (BoardCell::Filled(c), PuzzleCell::Fillable(correct_char)) => {
                c.to_ascii_lowercase() == correct_char.to_ascii_lowercase()
            }
            (BoardCell::Blocked, PuzzleCell::Blocked) => true,
            (BoardCell::Empty, PuzzleCell::Fillable(_)) => false,
            _ => false,
        }
    }
}

#[cfg(test)]
#[path = "player_board_tests.rs"]
mod player_board_tests;
