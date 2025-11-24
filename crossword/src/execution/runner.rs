use super::{CommandExecutor, KeyParser};
use crate::view;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{self, stdout};

pub struct Runner {
    pub parser: KeyParser,
    pub executor: CommandExecutor,
}

impl Runner {
    pub fn new(parser: KeyParser, executor: CommandExecutor) -> Self {
        Self { parser, executor }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = ratatui::init();

        // Main loop
        while self.executor.is_running() {
            // Render
            terminal.draw(|frame| {
                view::render(frame, self.executor.get_board());
            })?;

            // Handle input
            if let Some(command) = self.parser.parse_key() {
                self.executor.execute(command);
            }

            self.executor.check_has_won();
        }

        // Restore terminal
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        ratatui::restore();

        if self.executor.check_has_won() {
            println!("Congratulations! You've completed the crossword puzzle!");
        }

        Ok(())
    }
}
