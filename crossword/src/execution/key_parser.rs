use ratatui::crossterm::event;

use crate::{execution::game_command::GameCommand, model::MoveDirection};

pub struct KeyParser {}

impl KeyParser {
    pub fn new() -> Self {
        KeyParser {}
    }

    pub fn parse_key(&self) -> Option<GameCommand> {
        let key_event = event::read().ok()?;

        match key_event {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Char(c) => Some(GameCommand::EnterChar(c)),
                event::KeyCode::Up => Some(GameCommand::MoveInDirection(MoveDirection::Up)),
                event::KeyCode::Down => Some(GameCommand::MoveInDirection(MoveDirection::Down)),
                event::KeyCode::Left => Some(GameCommand::MoveInDirection(MoveDirection::Left)),
                event::KeyCode::Right => Some(GameCommand::MoveInDirection(MoveDirection::Right)),
                event::KeyCode::Backspace => Some(GameCommand::DeleteChar),
                event::KeyCode::Tab => Some(GameCommand::MoveToNextWord),
                event::KeyCode::Enter => Some(GameCommand::SwapDirection),
                event::KeyCode::Esc => Some(GameCommand::Quit),
                _ => None,
            },
            _ => None,
        }
    }
}
