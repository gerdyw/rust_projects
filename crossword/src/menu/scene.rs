use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::menu::MainMenuAction;

pub struct MainMenuScene {}

impl MainMenuScene {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> MainMenuAction {}
}
