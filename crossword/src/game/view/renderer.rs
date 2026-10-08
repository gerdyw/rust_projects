use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
};

use crate::{
    game::model::{BoardCell, BoardDirection, Coordinate, PlayerBoard},
    game::view::style::Theme,
};

pub struct Renderer {
    theme: Theme,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            theme: Theme::default(),
        }
    }

    pub fn render(&self, frame: &mut Frame, board: &PlayerBoard) {
        let area = frame.area();

        // Split the screen into main area and footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0), // Main content
                // Footer: height 5 -> 3 lines of content after borders, enough for wrapped clue
                Constraint::Length(5),
            ])
            .split(area);

        self.render_grid(frame, chunks[0], board);
        self.render_footer(frame, chunks[1], board);
        // If the player has completed the puzzle, show a congratulations overlay
        if board.has_won() {
            self.render_congratulations(frame, area, board);
        }
    }

    fn render_congratulations(&self, frame: &mut Frame, area: Rect, _board: &PlayerBoard) {
        // Compute popup dimensions (leave some padding)
        let popup_width = area.width.saturating_sub(4).max(10);
        let popup_height = std::cmp::min(area.height.saturating_sub(6), area.height / 2).max(3);

        // Center the popup
        let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        // Draw the popup border
        let border_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        frame.render_widget(border_block, popup_area);

        // Render the big text inside the popup with 1-cell padding
        if popup_area.width > 2 && popup_area.height > 2 {
            let inner = Rect {
                x: popup_area.x + 1,
                y: popup_area.y + 1,
                width: popup_area.width.saturating_sub(2),
                height: popup_area.height.saturating_sub(2),
            };

            let text = format!(
                "Winner! Time: {:.2?}",
                _board.get_total_time().unwrap_or(Duration::ZERO)
            );

            let paragraph = Paragraph::new(Span::styled(
                text,
                Style::default().fg(Color::Yellow).bold(),
            ))
            .alignment(Alignment::Center);

            frame.render_widget(paragraph, inner);
        }
    }

    fn render_grid(&self, frame: &mut Frame, area: Rect, board: &PlayerBoard) {
        let cursor = board.get_cursor();
        let width = board.width();
        let height = board.height();
        let elapsed_time = board.get_elapsed_time();

        // Get the current word to highlight
        let current_word = board.get_current_word();

        // Build the table rows
        let rows: Vec<Row> = (0..height)
            .map(|row| {
                let cells: Vec<Cell> = (0..width)
                    .map(|col| {
                        let coord = Coordinate::new(col as isize, row as isize);
                        let cell = board.get(coord);
                        let is_cursor = cursor == coord;

                        // Check if this cell is part of the current word
                        let in_current_word = current_word
                            .as_ref()
                            .map(|word| word.contains(&coord))
                            .unwrap_or(false);

                        let clue_number = board.get_clue_number_at(coord);

                        self.render_cell(cell, is_cursor, in_current_word, clue_number)
                    })
                    .collect();

                Row::new(cells).height(1)
            })
            .collect();

        // Create column constraints (3 chars per cell)
        let widths = vec![Constraint::Length(3); width];

        let table = Table::new(rows, widths)
            .block(
                Block::default()
                    .title(format!(
                        "{:02}:{:02} | Cursor: ({}, {})",
                        elapsed_time.as_secs() / 60,
                        elapsed_time.as_secs() % 60,
                        cursor.col,
                        cursor.row,
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            )
            .column_spacing(0);

        // Compute a tight area for the table so the borders fit around the grid
        let cell_width: u16 = 3;
        let content_width = cell_width.saturating_mul(width as u16);
        // +2 for left/right borders
        let mut table_width = content_width.saturating_add(2);
        if table_width > area.width {
            table_width = area.width;
        }

        // Height: rows + 2 for top/bottom borders
        let mut table_height = (height as u16).saturating_add(2);
        if table_height > area.height {
            table_height = area.height;
        }

        // Center the table inside the given area
        let table_x = area.x + (area.width.saturating_sub(table_width)) / 2;
        let table_y = area.y + (area.height.saturating_sub(table_height)) / 2;

        let table_area = Rect {
            x: table_x,
            y: table_y,
            width: table_width,
            height: table_height,
        };

        frame.render_widget(table, table_area);
    }

    fn render_cell(
        &self,
        cell: BoardCell,
        is_cursor: bool,
        in_current_word: bool,
        clue_number: Option<usize>,
    ) -> Cell<'static> {
        let text = if cell.is_empty()
            && let Some(number) = clue_number
        {
            format!("{:>2} ", number)
        } else {
            cell.to_string()
        };

        let style = if is_cursor {
            self.theme.cursor
        } else if in_current_word {
            self.theme.current_word
        } else {
            match cell {
                BoardCell::Filled(_) => self.theme.filled,
                BoardCell::Blocked => self.theme.blocked,
                BoardCell::Empty => {
                    if clue_number.is_some() {
                        self.theme.clue_number
                    } else {
                        self.theme.empty
                    }
                }
            }
        };

        Cell::from(text).style(style)
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, board: &PlayerBoard) {
        let cursor = board.get_cursor();

        // Get current word once
        let maybe_word = board.get_current_word();

        // Build the footer text: first line is the clue (with number styled yellow and length in brackets),
        let text = if let Some(word) = maybe_word.as_ref() {
            let mut clue_line = Self::clue_spans(&word.clue);
            clue_line.push(Span::styled(
                format!(" ({})", word.length()),
                Style::default().fg(Color::Yellow).bold(),
            ));

            let pos_line = Line::from(vec![
                Span::styled("Position: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("({}, {}) ", cursor.col, cursor.row),
                    Style::default().fg(Color::White),
                ),
            ]);

            vec![Line::from(clue_line), pos_line]
        } else {
            vec![
                Line::from("No word at current position"),
                Line::from(vec![
                    Span::styled("Position: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("({}, {}) ", cursor.col, cursor.row),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled("| ", Style::default().fg(Color::Gray)),
                    Span::styled("TAB", Style::default().fg(Color::Green).bold()),
                    Span::styled(": swap direction | ", Style::default().fg(Color::Gray)),
                    Span::styled("ESC", Style::default().fg(Color::Red).bold()),
                    Span::styled(": quit", Style::default().fg(Color::Gray)),
                ]),
            ]
        };

        // Compute direction arrow and label and put them into the footer block title
        let dir = board.get_direction();
        let dir_arrow = match dir {
            BoardDirection::Across => "→",
            BoardDirection::Down => "↓",
        };
        let dir_label = match dir {
            BoardDirection::Across => "Across",
            BoardDirection::Down => "Down",
        };

        // Footer title: show the clue number and direction (number is not styled here; only the
        // in-content number will be yellow). If no current word, show only the direction.
        let footer_title = if let Some(word) = maybe_word.as_ref() {
            format!("{} {}", word.clue_number, dir_label)
        } else {
            format!("Current Clue {} {}", dir_arrow, dir_label)
        };

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(footer_title))
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    }

    fn clue_spans(clue: &str) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        let mut rest = clue;
        let mut italic = false;

        while let Some(tag_pos) = rest.find(if italic { "</i>" } else { "<i>" }) {
            if tag_pos > 0 {
                spans.push(Span::raw(rest[..tag_pos].to_string()));
            }
            rest = &rest[tag_pos + if italic { 4 } else { 3 }..];
            italic = !italic;
            if italic {
                // capture until closing tag or end
                if let Some(close_pos) = rest.find("</i>") {
                    spans.push(Span::styled(
                        rest[..close_pos].to_string(),
                        Style::default().italic(),
                    ));
                    rest = &rest[close_pos + 4..];
                    italic = false;
                } else {
                    // no closing tag; italicize the rest
                    spans.push(Span::styled(rest.to_string(), Style::default().italic()));
                    rest = "";
                    break;
                }
            }
        }
        if !rest.is_empty() {
            spans.push(Span::raw(rest.to_string()));
        }
        spans
    }
}
