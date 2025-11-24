use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
};
use tui_big_text::{BigText, PixelSize};

use crate::model::{BoardCell, BoardDirection, Coordinate, PlayerBoard};

pub fn render(frame: &mut Frame, board: &PlayerBoard) {
    let area = frame.area();

    // Split the screen into main area and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer for clue
        ])
        .split(area);

    render_grid(frame, chunks[0], board);
    render_footer(frame, chunks[1], board);
    // If the player has completed the puzzle, show a congratulations overlay
    if board.has_won() {
        render_congratulations(frame, area, board);
    }
}

fn render_congratulations(frame: &mut Frame, area: Rect, _board: &PlayerBoard) {
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

fn render_grid(frame: &mut Frame, area: Rect, board: &PlayerBoard) {
    let cursor = board.get_cursor();
    let size: usize = board.size();
    let elapsed_time = board.get_elapsed_time();

    // Get the current word to highlight
    let current_word = board.get_current_word();

    // Build the table rows
    let rows: Vec<Row> = (0..size)
        .map(|row| {
            let cells: Vec<Cell> = (0..size)
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

                    let (content, style) = match cell {
                        BoardCell::Filled(ch) => {
                            let mut style = Style::default().fg(Color::White);
                            if is_cursor {
                                style = style.bg(Color::Yellow).fg(Color::Black).bold();
                            } else if in_current_word {
                                style = style.bg(Color::Cyan).fg(Color::Black);
                            }
                            (format!(" {} ", ch.to_uppercase()), style)
                        }
                        BoardCell::Empty => {
                            let mut style = Style::default();
                            if is_cursor {
                                style = style.bg(Color::Yellow);
                            } else if in_current_word {
                                style = style.bg(Color::Cyan);
                            }
                            if let Some(number) = clue_number {
                                (format!("{:>2} ", number), style.fg(Color::LightCyan).bold())
                            } else {
                                ("   ".to_string(), style)
                            }
                        }
                        BoardCell::Blocked => {
                            ("███".to_string(), Style::default().fg(Color::DarkGray))
                        }
                    };

                    Cell::from(content).style(style)
                })
                .collect();

            Row::new(cells).height(1)
        })
        .collect();

    // Create column constraints (3 chars per cell)
    let widths = vec![Constraint::Length(3); size];

    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .title(format!(
                    "{:02}:{:02} ",
                    elapsed_time.as_secs() / 60,
                    elapsed_time.as_secs() % 60
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .column_spacing(0);

    // Compute a tight area for the table so the borders fit around the grid
    let cell_width: u16 = 3;
    let content_width = cell_width.saturating_mul(size as u16);
    // +2 for left/right borders
    let mut table_width = content_width.saturating_add(2);
    if table_width > area.width {
        table_width = area.width;
    }

    // Height: rows + 2 for top/bottom borders
    let mut table_height = (size as u16).saturating_add(2);
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

fn render_footer(frame: &mut Frame, area: Rect, board: &PlayerBoard) {
    let cursor = board.get_cursor();

    // Get current word once
    let maybe_word = board.get_current_word();

    // Build the footer text: first line is the clue (with number styled yellow and length in brackets),
    // second line contains position and key hints.
    let text = if let Some(word) = maybe_word.as_ref() {
        vec![
            // Line::from(vec![Span::raw(format!(
            //     "{} ({})",
            //     word.clue,
            //     word.length()
            // ))]),
            Line::from(vec![
                // Span::styled(
                //     format!("{}. ", word.clue_number),
                //     Style::default().fg(Color::Yellow).bold(),
                // ),
                // Span::raw(format!("{} [{}]", word.clue, word.length())),
                Span::raw(word.clue.clone()),
                Span::styled(
                    format!(" ({})", word.length()),
                    Style::default().fg(Color::Yellow).bold(),
                ),
            ]),
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
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
