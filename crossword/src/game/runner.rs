use crate::game::view::renderer::Renderer;

use super::execution::{CommandExecutor, KeyParser};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::ExecutableCommand;
use std::io::{self, stdout};

pub struct GameScene {
    parser: KeyParser,
    executor: CommandExecutor,
    renderer: Renderer,
}

impl GameScene {
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
