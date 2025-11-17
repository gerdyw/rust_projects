use ratatui::{
    Frame,
    widgets::{Block, Row, Table},
};

use crate::model::{Model, board::Tile};

impl Model {
    pub fn draw(&self, frame: &mut Frame) {
        let rows = self.player_board.grid.row_iter().map(|row| {
            let cells = row.iter().map(|tile| match tile {
                Tile::Filled(c) => c.to_string(),
                Tile::Empty => " ".to_string(),
                Tile::Blocked => " ".to_string(),
            });
            Row::new(cells)
        });
        let row_lengths = vec![self.puzzle.size as u16; self.puzzle.size];
        let table = Table::new(rows, row_lengths);
        let block = Block::bordered().title("Crossword Puzzle");
        let table = table.block(block);
        frame.render_widget(table, frame.area());
    }
}
