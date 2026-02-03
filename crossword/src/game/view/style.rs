use ratatui::style::{Color, Style, Stylize};

pub struct Theme {
    pub border: Style,
    pub cursor: Style,
    pub current_word: Style,
    pub filled: Style,
    pub empty: Style,
    pub blocked: Style,
    pub clue_number: Style,
}
impl Default for Theme {
    fn default() -> Theme {
        Theme {
            border: Style::default().fg(Color::White),
            cursor: Style::default().bg(Color::Yellow).fg(Color::Black).bold(),
            current_word: Style::default().bg(Color::Cyan).fg(Color::Black),
            filled: Style::default().fg(Color::White),
            empty: Style::default(),
            blocked: Style::default().fg(Color::DarkGray),
            clue_number: Style::default().fg(Color::LightCyan).bold(),
        }
    }
}
