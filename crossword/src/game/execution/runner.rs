use crate::view::renderer::Renderer;

use super::{CommandExecutor, KeyParser};
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{self, stdout};

pub struct Runner {
    pub parser: KeyParser,
    pub executor: CommandExecutor,
    pub renderer: Renderer,
}

impl Runner {
    pub fn new(parser: KeyParser, executor: CommandExecutor, renderer: Renderer) -> Self {
        Self {
            parser,
            executor,
            renderer,
        }
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
                self.renderer.render(frame, self.executor.get_board());
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
