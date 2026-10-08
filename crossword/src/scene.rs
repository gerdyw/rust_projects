use std::io::{self, Stdout};

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{game::GameScene, menu::scene::MainMenuScene};

pub enum Scene {
    Game(GameScene),
    MainMenu(MainMenuScene),
    Paused,
    Completed,
}

pub enum SceneTransition {
    ToGame { game_file_path: String },
    ToMainMenu,
    ToPaused,
    Exit,
}

pub struct SceneRunner {
    current_scene: Scene,
}

impl SceneRunner {
    pub fn new(game_scene: GameScene) -> Self {
        Self {
            current_scene: Scene::Game(game_scene),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            match &mut self.current_scene {
                Scene::Game(game_scene) => {
                    game_scene.run(terminal)?;
                    // For now, game completion ends the app
                    // In the future, can transition to a different scene
                    break;
                }
                Scene::MainMenu(main_menu_scene) => {
                    // TODO: Implement menu runner
                    break;
                }
                Scene::Paused => {
                    // TODO: Implement pause menu runner
                    break;
                }
                Scene::Completed => {
                    // TODO: Implement completion screen runner
                    break;
                }
            }
        }
        Ok(())
    }
}
