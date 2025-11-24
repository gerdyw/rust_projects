use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::Coordinate;
use crate::{
    execution::game_command::GameCommand,
    model::{BoardCell, MoveDirection, PlayerBoard},
};

pub struct CommandExecutor {
    game_state: PlayerBoard,
    running: bool,
}

impl CommandExecutor {
    pub fn new(board: PlayerBoard) -> Self {
        CommandExecutor {
            game_state: board,
            running: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_board(&self) -> &PlayerBoard {
        &self.game_state
    }

    pub fn execute(&mut self, command: GameCommand) {
        let mut command = command;
        if self.check_has_won() {
            self.game_state.mark_as_won();
            self.quit();
            return;
        }

        while let Some(next_command) = self.execute_command(command) {
            command = next_command;
        }
    }
    pub fn execute_command(&mut self, command: GameCommand) -> Option<GameCommand> {
        log_command(&command);
        match command {
            GameCommand::MoveInDirection(direction) => self.move_direction(direction),
            GameCommand::SwapDirection => self.swap_direction(),
            GameCommand::EnterChar(c) => self.enter_char(c),
            GameCommand::MoveForward => self.move_forward(),
            GameCommand::MoveToNextWord => self.move_to_next_open_word(),
            GameCommand::MoveToNextEmptyCell => self.move_to_next_empty_cell(),
            GameCommand::DeleteChar => self.delete_char(),
            GameCommand::Quit => self.quit(),
            GameCommand::MoveToNextPlayableCell => self.move_to_next_playable_cell(),
            GameCommand::MoveTo(coordinate) => self.move_to(coordinate),
        }
    }

    pub fn check_has_won(&mut self) -> bool {
        let has_won = self.game_state.has_won();
        if has_won {
            self.game_state.mark_as_won();
        }
        has_won
    }

    fn move_direction(&mut self, direction: MoveDirection) -> Option<GameCommand> {
        let current_pos = self.game_state.get_cursor();
        self.game_state.move_cursor(direction);
        match current_pos.order_in_move_dir(&self.game_state.get_cursor(), direction) {
            Ordering::Greater => Some(GameCommand::SwapDirection),
            _ => None,
        }
    }

    fn move_to(&mut self, coordinate: Coordinate) -> Option<GameCommand> {
        self.game_state.move_to(coordinate).unwrap();
        None
    }

    fn move_forward(&mut self) -> Option<GameCommand> {
        Some(GameCommand::MoveInDirection(
            self.game_state.get_direction().into(),
        ))
    }

    fn swap_direction(&mut self) -> Option<GameCommand> {
        self.game_state.swap_direction();
        None
    }

    fn enter_char(&mut self, c: char) -> Option<GameCommand> {
        let overwrite_mode = matches!(self.game_state.get_current(), BoardCell::Filled(_));
        self.game_state.write_to_cell(c);

        if overwrite_mode {
            return Some(GameCommand::MoveForward);
        }

        let Some(mut word_iter) = self.game_state.current_word_iter() else {
            return Some(GameCommand::MoveToNextEmptyCell);
        };

        let empty_cell =
            word_iter.find(|coord| matches!(self.game_state.get(*coord), BoardCell::Empty));

        if let Some(cell) = empty_cell {
            return Some(GameCommand::MoveTo(cell));
        }

        Some(GameCommand::MoveToNextEmptyCell)
    }

    fn move_to_next_empty_cell(&mut self) -> Option<GameCommand> {
        self.game_state.move_to_next_empty_cell();
        None
    }

    fn move_to_next_open_word(&mut self) -> Option<GameCommand> {
        self.game_state.move_to_next_open_word();
        None
    }

    fn delete_char(&mut self) -> Option<GameCommand> {
        if self.game_state.get_current() == BoardCell::Empty {
            self.game_state.move_cursor(
                self.game_state
                    .get_direction()
                    .to_move_direction()
                    .reverse(),
            );
        }

        self.game_state.clear_cell();
        None
    }

    fn quit(&mut self) -> Option<GameCommand> {
        self.running = false;
        None
    }

    fn move_to_next_playable_cell(&mut self) -> Option<GameCommand> {
        self.game_state.move_to_next_open_word();
        None
    }
}

fn log_command(command: &GameCommand) {
    // Append the incoming command to a simple log file for live-tail debugging.
    // Non-fatal: ignore any I/O errors so the game keeps running.
    let _ = std::fs::create_dir_all("logs");
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/commands.log")
    {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(f, "{} {:?}", ts, command);
    }
}
