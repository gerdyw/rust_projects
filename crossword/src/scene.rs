use std::io;

use crate::game;

pub enum Scene {
    Game(game::GameScene),
    Menu,
    Paused,
    Completed,
}

pub struct SceneRunner {
    current_scene: Scene,
}

impl SceneRunner {
    pub fn new(game_scene: game::GameScene) -> Self {
        Self {
            current_scene: Scene::Game(game_scene),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            match &mut self.current_scene {
                Scene::Game(game_scene) => {
                    game_scene.run()?;
                    // For now, game completion ends the app
                    // In the future, can transition to a different scene
                    break;
                }
                Scene::Menu => {
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
