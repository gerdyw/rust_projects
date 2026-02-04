use std::io::{self, Stdout};

use crossword::game::GameScene;
use crossword::game::execution::puzzle_parser::{from_toml_file, parse_ipuz_to_puzzle};
use crossword::game::execution::{CommandExecutor, KeyParser, PuzzleType};
use crossword::game::model::PlayerBoard;
use crossword::game::view::renderer::Renderer;
use crossword::scene::SceneRunner;
use ratatui::prelude::CrosstermBackend;
use ratatui::{Terminal, prelude};

fn main() -> std::io::Result<()> {
    // Parse the puzzle file path from command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <puzzle-file-path>", args[0]);
        eprintln!("Example: {} puzzles/BB-8.ipuz", args[0]);
        std::process::exit(1);
    }

    let puzzle_path = &args[1];
    let puzzle_type = PuzzleType::from_file_path(puzzle_path).expect("puzzle type unsupported");

    // Read the puzzle file into a String
    let puzzle_content = std::fs::read_to_string(puzzle_path)?;

    let puzzle = match puzzle_type {
        PuzzleType::Ipuz => parse_ipuz_to_puzzle(&puzzle_content),
        PuzzleType::Toml => from_toml_file(&puzzle_path),
    }
    .expect("error parsing puzzle");

    let board = PlayerBoard::from_puzzle(puzzle);
    let parser = KeyParser::new();
    let executor = CommandExecutor::new(board);
    let renderer = Renderer::new();
    let game_scene = GameScene::new(parser, executor, renderer);
    let mut scene_runner = SceneRunner::new(game_scene);

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();

    scene_runner.run(&mut terminal)?;

    Ok(())
}
