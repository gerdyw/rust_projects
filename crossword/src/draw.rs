use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::model::{Model, board::Tile};

impl Model {
    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // Create layout with border
        let block = Block::default()
            .title("Crossword Puzzle")
            .borders(Borders::ALL);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Calculate cell dimensions
        let cell_width = 3; // Width for each cell (char + padding)
        let cell_height = 1;

        // Create the grid
        let rows: Vec<Row> = self
            .player_board
            .grid
            .row_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let cells: Vec<Cell> = row
                    .iter()
                    .enumerate()
                    .map(|(col_idx, tile)| {
                        let is_selected = self.player_board.pos.row == row_idx
                            && self.player_board.pos.col == col_idx;

                        let (content, style) = match tile {
                            Tile::Filled(c) => {
                                let style = if is_selected {
                                    Style::default()
                                        .fg(Color::Black)
                                        .bg(Color::Yellow)
                                        .add_modifier(Modifier::BOLD)
                                } else {
                                    Style::default().fg(Color::White)
                                };
                                (format!(" {} ", c.to_uppercase()), style)
                            }
                            Tile::Empty => {
                                let style = if is_selected {
                                    Style::default().fg(Color::Black).bg(Color::Yellow)
                                } else {
                                    Style::default()
                                };
                                (String::from("   "), style)
                            }
                            Tile::Blocked => {
                                (String::from(" █ "), Style::default().fg(Color::DarkGray))
                            }
                        };

                        Cell::from(content).style(style)
                    })
                    .collect();

                Row::new(cells).height(cell_height)
            })
            .collect();

        // Create column constraints
        let widths = vec![Constraint::Length(cell_width); self.puzzle.size];

        let table = Table::new(rows, widths).column_spacing(0);

        frame.render_widget(table, inner_area);

        // Show current direction at the bottom
        let direction_text = format!(
            "Direction: {} | Position: ({}, {}) | ESC to quit, TAB to swap direction",
            match self.player_board.direction {
                crate::model::common::BoardDirection::Across => "→ Across",
                crate::model::common::BoardDirection::Down => "↓ Down",
            },
            self.player_board.pos.col,
            self.player_board.pos.row
        );

        let info_area = Rect {
            x: area.x,
            y: area.y + area.height - 1,
            width: area.width,
            height: 1,
        };

        let info = Paragraph::new(direction_text).style(Style::default().fg(Color::Cyan));

        frame.render_widget(info, info_area);
    }
}
