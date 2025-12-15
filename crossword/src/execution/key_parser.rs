use std::time::Duration;

use ratatui::crossterm::event::{self, Event::Key, KeyCode};

use crate::{execution::game_command::GameCommand, model::MoveDirection};

pub struct KeyParser {}

impl KeyParser {
    pub fn new() -> Self {
        KeyParser {}
    }

    pub fn parse_key(&self) -> Option<GameCommand> {
        if !event::poll(Duration::from_millis(250)).ok()? {
            return None;
        }

        let key_event = event::read().ok()?;

        match key_event {
            Key(key) => match key.code {
                KeyCode::Char(c) if c.is_alphabetic() => Some(GameCommand::EnterChar(c)),
                KeyCode::Char(c) if c == ' ' => Some(GameCommand::MoveForward),
                KeyCode::Up => Some(GameCommand::MoveInDirection(MoveDirection::Up)),
                KeyCode::Down => Some(GameCommand::MoveInDirection(MoveDirection::Down)),
                KeyCode::Left => Some(GameCommand::MoveInDirection(MoveDirection::Left)),
                KeyCode::Right => Some(GameCommand::MoveInDirection(MoveDirection::Right)),
                KeyCode::Backspace => Some(GameCommand::DeleteChar),
                KeyCode::Tab => Some(GameCommand::SwapDirection),
                KeyCode::BackTab => Some(GameCommand::MoveToPreviousOpenWord),
                KeyCode::Enter => Some(GameCommand::MoveToNextOpenWord),
                KeyCode::Esc => Some(GameCommand::Quit),
                _ => None,
            },
            _ => None,
        }
    }
}
