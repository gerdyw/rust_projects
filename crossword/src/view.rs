use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
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
    // Use `tui-big-text` to render a large congratulations string
    // We'll render the big text inside a bordered popup and center it.
    let text = "Winner!";

    // Compute popup dimensions (leave some padding)
    // Target a popup that is about half the screen or smaller
    let popup_width = area.width - 4;
    let popup_height = std::cmp::min(area.height.saturating_sub(6), area.height / 2);

    // Center the popup
    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    let title = Span::styled(
        "Winner!",
        Style::default().fg(Color::White).bg(Color::Green).bold(),
    );

    // Draw the popup border
    let border_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    frame.render_widget(border_block, popup_area);

    // Render the big text inside the popup with 1-cell padding
    if popup_area.width > 2 && popup_area.height > 2 {
        let inner = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + 1,
            width: popup_area.width - 2,
            height: popup_area.height - 2,
        };

        // Create the BigText widget via its builder and render it into the inner area.
        let big = BigText::builder()
            .pixel_size(PixelSize::Full)
            .centered()
            .style(Style::default().fg(Color::Yellow).bold())
            .lines(vec![text.into()])
            .build();

        frame.render_widget(big, inner);
    }
}

fn render_grid(frame: &mut Frame, area: Rect, board: &PlayerBoard) {
    let cursor = board.get_cursor();
    let direction = board.get_direction();
    let size = board.size();

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

    let title = format!(
        "Crossword Puzzle {} {}",
        match direction {
            BoardDirection::Across => "→",
            BoardDirection::Down => "↓",
        },
        match direction {
            BoardDirection::Across => "Across",
            BoardDirection::Down => "Down",
        }
    );

    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .column_spacing(0);

    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, area: Rect, board: &PlayerBoard) {
    let cursor = board.get_cursor();

    // Build the footer text
    let text = if let Some(word) = board.get_current_word() {
        vec![
            Line::from(vec![
                Span::styled(
                    format!("{}. ", word.clue_number),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw(&word.clue),
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

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Current Clue"))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
