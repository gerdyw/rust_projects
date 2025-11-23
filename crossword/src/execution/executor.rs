use std::cmp::Ordering;

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
        if self.has_won() {
            self.quit();
            return;
        }

        while let Some(next_command) = self.execute_command(command) {
            command = next_command;
        }
    }
    pub fn execute_command(&mut self, command: GameCommand) -> Option<GameCommand> {
        match command {
            GameCommand::MoveInDirection(direction) => self.move_direction(direction),
            GameCommand::SwapDirection => self.swap_direction(),
            GameCommand::EnterChar(c) => self.enter_char(c),
            GameCommand::MoveForward => self.move_forward(),
            GameCommand::MoveToNextWord => self.move_to_next_open_word(),
            GameCommand::MoveToNextEmptyCell => self.move_to_next_empty_cell(),
            GameCommand::DeleteChar => self.delete_char(),
            GameCommand::Quit => self.quit(),
        }
    }

    pub fn has_won(&self) -> bool {
        self.game_state.has_won()
    }

    fn move_direction(&mut self, direction: MoveDirection) -> Option<GameCommand> {
        let current_pos = self.game_state.get_cursor();
        self.game_state.move_cursor(direction);
        match current_pos.order_in_move_dir(&self.game_state.get_cursor(), direction) {
            Ordering::Greater => Some(GameCommand::SwapDirection),
            _ => None,
        }
    }

    fn move_forward(&mut self) -> Option<GameCommand> {
        self.game_state.move_forward();
        None
    }

    fn swap_direction(&mut self) -> Option<GameCommand> {
        self.game_state.swap_direction();
        None
    }

    fn enter_char(&mut self, c: char) -> Option<GameCommand> {
        let overwrite_mode = matches!(self.game_state.get_current(), BoardCell::Filled(_));
        self.game_state.write_to_cell(c);
        if overwrite_mode {
            Some(GameCommand::MoveForward)
        } else {
            Some(GameCommand::MoveToNextEmptyCell)
        }
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
            self.game_state.move_backward();
        }
        self.game_state.clear_cell();
        None
    }

    fn quit(&mut self) -> Option<GameCommand> {
        self.running = false;
        None
    }
}
